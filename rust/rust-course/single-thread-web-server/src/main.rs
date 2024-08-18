use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // ::bind is similar with ::new, returning a Result<TcpListener, io::Error>
    let listener = TcpListener::bind("127.0.0.1:5566").unwrap();
    // incoming returns an Iterator<Item = Result<TcpStream, io::Error>>
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("Connection established from {}", stream.peer_addr().unwrap());
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // the ending of the http request is an empty line
    let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    // println!("Request: {:#?}", http_request);

    // http request:
    // Method Request-URI HTTP-Version
    // headers CRLF
    // body (GET Method no body)

    // http response:
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // body (GET Method no body)

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
