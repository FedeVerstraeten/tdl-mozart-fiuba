// IP address = 127.0.0.1 (localhost)
// Port = 7878
//
// HTTP is normally accepted on this port,
// and 7878 is rust typed on a telephone

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];

  // We pass the buffer to stream.read, which will read 
  // bytes from the TcpStream and put them in the buffer. 
  stream.read(&mut buffer).unwrap();

//    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
//    let response = "HTTP/1.1 200 OK\r\n\r\n";

  // we transform get into a byte string by adding the b"" 
  // byte string syntax
  let get = b"GET / HTTP/1.1\r\n";
  
  let path_hello = "200.html";

  let path_404 = "404.html";
  
if buffer.starts_with(get){

    let contents = fs::read_to_string(path_hello).unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }else {

    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let contents = fs::read_to_string(path_404).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }
}
