use std::net::{ SocketAddr, TcpStream, TcpListener};
use std::io::{Read, Write};

fn main() {
    // Create a listening TCP server on port 9000
    let handle = std::thread::spawn(move || {
        let server = TcpListener::bind("127.0.0.1:9000").expect("server bind");
        let (mut stream, _) = server.accept().expect("server accept");

        let mut buf = [0; 19];
        stream.read(&mut buf).expect("server read");
        println!("server_recv {:?}",buf);

        stream.write_all(b"server2client").expect("server write");
    });

    handle.join().unwrap();
}
