use std::net::{ SocketAddr, TcpStream, TcpListener};
use std::io::{Read, Write};

// Create a listening TCP server on port 9000
fn main() {
    let handle = std::thread::spawn(move || {

        // nitro enclave stops at bind
        let server = TcpListener::bind("127.0.0.1:9000").expect("server bind");
        let (mut stream, _) = server.accept().expect("server accept");

        let mut buf = [0; 19];
        stream.read(&mut buf).expect("server read");
        println!("server_recv {:?}",buf);

        stream.write_all(b"server2client").expect("server write");
    });

    handle.join().unwrap();
}
