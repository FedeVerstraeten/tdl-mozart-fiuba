// IP address = 127.0.0.1 (localhost)
// Port = 7878
//
// HTTP is normally accepted on this port,
// and 7878 is rust typed on a telephone

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration; 

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
  //let sleep = b"GET /sleep HTTP/1.1\r\n";
 
  let path_hello = "200.html";

  let path_404 = "404.html";

   // return values for the status line and filename in a tuple
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", path_hello)
//    } else if buffer.starts_with(sleep) {
//        thread::sleep(Duration::from_secs(5));
//        ("HTTP/1.1 200 OK\r\n\r\n", path_hello)
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", path_404)
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();  
}
