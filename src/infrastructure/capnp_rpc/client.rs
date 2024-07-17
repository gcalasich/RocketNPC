use crate::hello_world_capnp::hello_world;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::net::ToSocketAddrs;

use futures::AsyncReadExt;

pub async fn run_client(  message: String ) ->Result<String, Box<dyn std::error::Error>> {
    let server_addr : String = "127.0.0.1:4000".to_string();
    let addr = server_addr
        .to_socket_addrs().unwrap()
        .next()
        .expect("could not parse address");       
         
         rocket::tokio::task::LocalSet::new()
            .run_until( async move {
                let stream = rocket::tokio::net::TcpStream::connect(&addr).await?;
                stream.set_nodelay(true).unwrap();
                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                let rpc_network = Box::new(twoparty::VatNetwork::new(
                    futures::io::BufReader::new(reader),
                    futures::io::BufWriter::new(writer),
                    rpc_twoparty_capnp::Side::Client,
                    Default::default(),
                ));
                let mut rpc_system = RpcSystem::new(rpc_network, None);
                let hello_world: hello_world::Client =
                    rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
    
                rocket::tokio::task::spawn_local(rpc_system);
                //handle.spawn(rpc_system);
                println!("entro ");
                let mut request = hello_world.say_hello_request();
                request.get().init_request().set_name(&message[..]);
    
                let reply = request.send().promise.await?;
                println!("se fue 2");
                let reply_message  = reply.get()?.get_reply()?.get_message()?.to_str()?;
                println!("received: {}", reply_message);
                Ok(reply_message.to_string())
            }).await
        }