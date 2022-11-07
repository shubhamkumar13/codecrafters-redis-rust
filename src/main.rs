#[allow(unused_imports)]
use core::panic;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;
#[allow(unused_imports)]
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    println!("Logs from your program will appear here!");
    let addr = "127.0.0.1:6379";

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let mut buffer: [u8; 1024] = [0; 1024];

                loop {
                    match s.read(&mut buffer) {
                        Ok(_) => {
                            let response = format!("+PONG\r\n");
                            s.write(response.as_bytes())?;
                        }
                        Err(e) => {
                            println!("Error reading from stream : {e:?}");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Couldn't accept client : {e:?}");
            }
        }
    }

    Ok(())
}
