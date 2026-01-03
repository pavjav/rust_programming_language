use std::{
    fs, io::{BufReader, prelude::*}, net::{TcpListener, TcpStream}
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    // for stream in listener.incoming().take(5) { // only allow 5 requests
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        //println!("Connection established!");
        pool.execute(|| { handle_connection(stream); });
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty()) // two newlines signal end of request
        .collect();
    //println!("Request: {:#?}", http_request);

    //let response = "HTTP/1.1 200 OK\r\n\r\n"; // Standard response
    let request_line = http_request[0].as_str();
    let (status, file_name) = 
        if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();
    let response = format!(
        "{status}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}