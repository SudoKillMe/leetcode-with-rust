
use tokio::io;
use tokio::net::TcpStream;
use futures::Future;
use tokio::prelude::*;
use std::io::Write;


fn main() {
    let addr = "127.0.0.1:1234".parse().unwrap();

    let future = TcpStream::connect(&addr)
        .and_then(|socket| {

//            let (reader, writer) = socket.split();


//            writer.write_all(b"ni hao shi jie");
//            io::write_all(socket, b"ni hao shi jie")

            let stream = std::io::BufReader::new(socket);
            io::lines(stream).for_each(|line| {
                println!("[client]: get lines from server: {}", line);
                Ok(())
            })
//            tokio::spawn(read_from_server);
//
//            io::write_all(writer, b"ni hao shi jie \n second line")
        })
        .map(|_| println!("complete"))
        .map_err(|_| println!("failed"));

    tokio::run(future);
}
