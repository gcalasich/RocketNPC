use std::{
    any::Any,
    collections::BTreeMap,
    sync::{atomic::AtomicU64, mpsc, Mutex, OnceLock},
    task::{Poll, Waker},
};

use capnp_rpc::*;
use futures::AsyncReadExt;
use tokio::net::ToSocketAddrs;

use crate::hello_world_capnp::hello_world;

type RpcResult = Box<dyn Any + Send>;

enum RpcCall {
    SayHelloRequest { message: String },
}

static RESULT_POOL: OnceLock<Mutex<BTreeMap<u64, (Option<RpcResult>, Waker)>>> = OnceLock::new();

struct ResultId(u64);

impl std::future::Future for ResultId {
    type Output = RpcResult;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mutex = RESULT_POOL.get().expect("RESULT_POOL not initialized");
        let Ok(mut result_pool) = mutex.try_lock() else {
            return Poll::Pending;
        };

        // insert waker
        if result_pool.get_mut(&self.0).is_none() {
            let _ = result_pool.insert(self.0, (None, cx.waker().clone()));
            return Poll::Pending;
        }

        // check for result
        match result_pool.remove(&self.0) {
            Some((Some(res), _)) => Poll::Ready(res),
            Some(_) => {
                unreachable!("A second poll indicates a value has been added")
            }
            None => unreachable!("If no result is pending, how did we skip the None check above?"),
        }
    }
}

#[derive(Debug)]
pub struct SerializingRpcClient {
    sender: mpsc::Sender<(u64, RpcCall)>,
    counter: AtomicU64,
}

impl SerializingRpcClient {
    pub async fn new(addr: impl ToSocketAddrs) -> SerializingRpcClient {
        // initialize result pool, once
        let _ = RESULT_POOL.set(Default::default());

        let stream = rocket::tokio::net::TcpStream::connect(&addr)
            .await
            .expect("Unable to open CapN'p TPC Stream");
        stream.set_nodelay(true).unwrap();

        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let rpc_network = Box::new(twoparty::VatNetwork::new(
            futures::io::BufReader::new(reader),
            futures::io::BufWriter::new(writer),
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));
        let mut rpc_system = RpcSystem::new(rpc_network, None);
        let hello_world: hello_world::Client =
            rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        // drive rpc system
        let (sender, receiver) = mpsc::channel();
        tokio::task::LocalSet::new()
            .run_until(async move {
                tokio::task::spawn_local(SerializingRpcClient::start(hello_world, receiver));
                tokio::task::spawn_local(rpc_system);
            })
            .await;

        SerializingRpcClient {
            sender,
            counter: 0.into(),
        }
    }

    async fn start(client: hello_world::Client, receiver: mpsc::Receiver<(u64, RpcCall)>) {
        println!("Started Sync task");

        // start RPC thread
        while let Ok((id, msg)) = receiver.recv() {
            println!("Got RpcCall #{}", id);

            match msg {
                RpcCall::SayHelloRequest { message } => {
                    let mut request = client.say_hello_request();
                    request.get().init_request().set_name(&message[..]);

                    let reply = request.send().promise.await.unwrap();
                    // the capnp-rpc crate kinda really sucks
                    let reply_message = reply
                        .get()
                        .unwrap()
                        .get_reply()
                        .unwrap()
                        .get_message()
                        .unwrap()
                        .to_str()
                        .unwrap();

                    let res = reply_message.to_string();
                    let r#box: Box<dyn Any + Send> = Box::new(res);

                    // send result
                    let mutex = RESULT_POOL.get().expect("RESULT_POOL not initialized");
                    let mut result_pool = mutex.lock().unwrap();
                    let (item, waker) = result_pool
                        .get_mut(&id)
                        .expect("Unable to get result object for SayHelloRequest call");

                    *item = Some(r#box);
                    waker.wake_by_ref();
                }
            };
        }

        println!("SerializingClient sync task has exited");
    }

    pub fn next_task_id(&self) -> u64 {
        loop {
            let id = self
                .counter
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            let mutex = RESULT_POOL.get().expect("RESULT_POOL not initialized");
            let results = mutex.lock().unwrap();

            // Avoid collisions
            if !results.contains_key(&id) {
                break id;
            }
        }
    }

    pub async fn say_hello_request(&self, message: String) -> Box<String> {
        let id = self.next_task_id();

        self.sender
            .send((id, RpcCall::SayHelloRequest { message }))
            .unwrap();
        println!("Sent RpcCall #{}", id);

        let res = ResultId(id).await;
        res.downcast::<String>().unwrap()
    }
}
