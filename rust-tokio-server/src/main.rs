use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    let addr = "127.0.0.1:1234".parse().unwrap();

    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(|socket| {

//        let (reader, writer) = socket.split();

        let content = "just for test \n now it is second \n now it is three".as_bytes();
        let test = io::write_all(socket,content)
            .then(|_| {
            Ok(())
        });

//        let amount = io::copy(reader, writer);
//        let msg = amount.then(|result| {
//
//            match result {
//                Ok((amount, _, _)) => println!("wrote {} bytes", amount),
//                Err(e) => println!("error: {}", e),
//            }
//
//            Ok(())
//        });
        tokio::spawn(test);
//
////        let writer_to_client = io::write_all(writer, b"i am come from server");
////        let server_msg = writer_to_client.then(|_| {
////            Ok(())
////        });
////        tokio::spawn(server_msg);
////        println!("[server] get data = {:?}", socket);
        Ok(())
    }).map_err(|err| {
        println!("[server] error = {:?}", err);
    });
    println!("server running on localhost:1234");

    tokio::run(server);
}
