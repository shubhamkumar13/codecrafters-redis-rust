#[allow(unused_imports)]
use core::panic;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;
#[allow(unused_imports)]
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let addr = "127.0.0.1:6379";

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.write_fmt(format_args!("+PONG\r\n")).unwrap();
            }
            Err(_) => (),
        }
    }

    Ok(())
    // match listener.accept() {
    //     Ok((_socket, addr)) => println!("accepted new client: {:?}", addr),
    //     Err(e) => println!("couldn't accept client: {:?}", e),
    // }
}
