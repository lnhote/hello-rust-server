use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // open a tcp connection, listen to the ip+port

    // unwrap will get the expected content when return value is OK
    // or panic when the return value is Err
    let listener = TcpListener::bind("127.0.0.1:8848").unwrap();
    for stream in listener.incoming() {
        let result = stream.unwrap();
        println!("Connection established! {:?}", result);
        handle_connection(result);
    }
    println!("exited");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    // convert the bytes in the buffer to a string
    // The lossy part of the name indicates the behavior of this function
    // when it sees an invalid UTF-8 sequence: it will replace the invalid sequence with �,
    // the U+FFFD REPLACEMENT CHARACTER
    println!("Request: {}", String::from_utf8_lossy(&buf[..]));

    if buf.starts_with(b"GET / HTTP/1.1\r\n") {
        // write a response
        let html = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", html);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let response = format!("{}{}", status_line, fs::read_to_string("404.html").unwrap());
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
