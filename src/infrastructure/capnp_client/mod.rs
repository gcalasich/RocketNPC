use std::error::Error;

use capnp_rpc::*;
use futures::AsyncReadExt;
use tokio::{
    net::ToSocketAddrs,
    sync::{Mutex, MutexGuard},
};

use crate::hello_world_capnp::hello_world;

#[repr(transparent)]
struct UnsafeSendCell<T>(T);

impl<T> UnsafeSendCell<T> {
    fn new(inner: T) -> Mutex<UnsafeSendCell<T>> {
        Mutex::new(UnsafeSendCell(inner))
    }
}

// SAFETY: Internally: capnp-rpc uses Rc<RefCell> that's why it doesn't implement Send. But if we wrap it in a Mutex, we'll be fine
unsafe impl<T> Send for UnsafeSendCell<T> {}

pub struct UnsafeClient {
    client: Mutex<UnsafeSendCell<hello_world::Client>>,
}

impl UnsafeClient {
    pub async fn new(addr: impl ToSocketAddrs) -> UnsafeClient {
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
        let client: hello_world::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        // drive rpc system
        tokio::task::LocalSet::new().run_until(rpc_system).await;

        UnsafeClient {
            client: UnsafeSendCell::new(client),
        }
    }

    pub async fn get(&self) -> MutexGuard<UnsafeSendCell<hello_world::Client>> {
        self.client.lock().await
    }

    pub async fn say_hello_request(&self, message: String) -> Result<String, Box<dyn Error>> {
        let client = self.get().await;

        let mut request = client.0.say_hello_request();
        request.get().init_request().set_name(&message[..]);

        // RESOLVE: even the return value is !Send, which is problematic
        let reply = request.send().promise.await?;
        let reply_message = reply.get()?.get_reply()?.get_message()?.to_str()?;
        Ok(reply_message.to_string())
    }
}
