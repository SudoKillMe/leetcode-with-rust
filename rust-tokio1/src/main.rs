#[macro_use]
extern crate futures;

use tokio::net::{ TcpStream, tcp::ConnectFuture };
use futures::{ Future, Async, Poll };

struct GetPeerAddr {
    connect: ConnectFuture
}

impl Future for GetPeerAddr {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        println!("poll...");
        match self.connect.poll() {
            Ok(Async::Ready(socket)) => {
                println!("peer address = {}", socket.peer_addr().unwrap());
                Ok(Async::Ready(()))
            },
            Ok(Async::NotReady) => {
                println!("not ready");
                Ok(Async::NotReady)
            },
            Err(e) => {
                println!("failed to connect: {}", e);
                Ok(Async::Ready(()))
            },
        }
        try_ready!()
    }
}

fn main() {

    let addr = "127.0.0.1:1234".parse().unwrap();
    let connect_future = TcpStream::connect(&addr);
    let get_peer_addr = GetPeerAddr {
        connect: connect_future
    };

    tokio::run(get_peer_addr);

}
