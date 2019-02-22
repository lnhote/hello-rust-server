use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

#[path="../lib.rs"]
mod pooling;

use pooling::ThreadPool;

fn main() {
    // open a tcp connection, listen to the ip+port

    // unwrap will get the expected content when return value is OK
    // or panic when the return value is Err
    let listener = TcpListener::bind("127.0.0.1:8848").unwrap();
    let pool = ThreadPool::new(4);
    // terminate after accepting 10 requests before gracefully shutting down the server
    for stream in listener.incoming().take(10) {
        let result = stream.unwrap();
        println!("Connection established! {:?}", result);
        pool.execute(|| {
            handle_connection(result);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    // convert the bytes in the buffer to a string
    // The lossy part of the name indicates the behavior of this function
    // when it sees an invalid UTF-8 sequence: it will replace the invalid sequence with �,
    // the U+FFFD REPLACEMENT CHARACTER
    println!("Request: {}", String::from_utf8_lossy(&buf[..]));

    let (status_line, filename) = if buf.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buf.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    // write a response
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
