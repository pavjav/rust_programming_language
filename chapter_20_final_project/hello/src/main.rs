use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream}
};

use hello::ThreadPool;

fn main() {
    // Create threadpool with 4 workers
    let pool = ThreadPool::new(4);
    // Create listener or else panic
    let tcp_listener = TcpListener::bind("127.0.0.1:7878");

    match tcp_listener {
        Ok(listener) => {
            for stream in listener.incoming() { 
            // Check if listener stream is valid, if not no execution is made
            // If valid we use the execute method and pass a closure executing the handle_connection() function
            
                match stream  {
                    Ok(res) => pool.execute(|| {handle_connection(res);}),
                    Err(err) => {
                        eprintln!("Failed listening with error : {}", err);
                        eprintln!("Continuing...");
                    }
                }
            }
        },
        Err(err) => {
            eprintln!("Unable to create listener, ensure address and port are valid.");
            eprintln!("{err}");
        }
    }
    
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    // BufReader turns a a TcpStream into an iterable
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines() // Lines<BufReader<&mut TcpStream>>
        .map(|result| 
            {
                result.unwrap_or_default() 
            }
        )//Iterator of strings
        .take_while(|line| !line.is_empty()) // Iterator of strings up until empty string
        .collect(); // finally turn into a vector of Strings
    //println!("Request: {:#?}", http_request); // debug the http_request

    //let response = "HTTP/1.1 200 OK\r\n\r\n"; // Standard response
    let request_line = http_request[0].as_str(); //First line with http request header
    // Serve site if GET / else return 404
    let (status, file_name) = 
        if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    // Get String from html file, or return "" on fail
    let contents = fs::read_to_string(file_name).unwrap_or_default();

    // Get length of response
    let length = contents.len();

    // Format response in HTTP standard
    let response = format!(
        "{status}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    // Write the response in byte buffer and unwrap the result
    stream
        .write_all(response.as_bytes())
        .unwrap();
}