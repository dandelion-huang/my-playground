use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use multi_thread_web_server::ThreadPool;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:5566").unwrap();
    // only accept 2 connections
    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();
        let pool = ThreadPool::new(4);
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("===SHUTTING DOWN===");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, file_name) = match buffer.as_slice() {
        request if request.starts_with(get) => {
            ("HTTP/1.1 200 OK", "hello.html")
        },
        request if request.starts_with(sleep) => {
            println!("sleep, zzz...");
            thread::sleep(Duration::from_secs(5));
            println!("wake up!");
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
