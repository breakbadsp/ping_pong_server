use std::arch::x86_64;
use std::collections::VecDeque;
use std::io::{ BufRead, BufReader, BufWriter, Write };
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let server = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server started listening on {}", server.local_addr().unwrap());
    let mut handlers = vec![];
    for stream_or_error in server.incoming() {
        match stream_or_error  {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());
                let handler = std::thread::spawn( || {
                    handle_connection( stream);
                });
                handlers.push(handler);
            }
            Err( _err ) => {
                
            }
        }
    }

    for handle in handlers {
        handle.join().unwrap();
    }
}

fn handle_connection(mut p_tcp_stream : TcpStream) {
    println!("handler created for connection from {}", p_tcp_stream.peer_addr().unwrap());
    let mut line = String::new();
    
    loop {
        line.clear();
        let mut stream_copy = p_tcp_stream.try_clone().unwrap();
        let mut buff_reader = BufReader::new(&mut stream_copy);
        let len_or_error = buff_reader.read_line(&mut line);
        line = line.trim_end().to_string();
        match len_or_error {
            Ok(len) => {
                println!("\nread {len} bytes of data {line}");
            }
            Err(err) => {
                println!("Error in reading the data from {}", p_tcp_stream.peer_addr().unwrap());
                ()
            }
        }
        if line.trim_end() == String::from("PING") {
            match p_tcp_stream.write_all(b"PONG\n") {
                Err(err) => {
                    println!("Error {} while writing data", err);

                }
                Ok(t) => {
                    println!("wrote data PONG");
                }
            }
        }
    }
}