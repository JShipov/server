mod lib;
use std::fs;

//into scope to get access to traits and types that
//let us read from and write to the stream
use std::io::prelude::*;

//standart library let listen Tcp connection
use std::net::TcpListener;
use std::net::TcpStream;
use hello::ThreadPool;

use std::thread;

use std::time::Duration;

fn main() {
    
    //listening for Tcp connection at the IP-adress 127.0.0.1, binding to port 7878
    //HTTP isn’t normally accepted on this port, server is unlikely to conflict with any other web server 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    //create a new thread pool with a configuarable number of threads  
    let pool = ThreadPool::new(4);

    //get the error about request
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

//multiply reguest
//response that will cause the server to sleep for 5 seconds before respond
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";   
   
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    
    //ger a response 
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

