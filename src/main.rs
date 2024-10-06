/*
1. Have your proxy server start up and listen for connections. In my case I decided to listen
on port 8989.
2. When a request is received parse it to extract the target host.
3. Create a new socket / connection to the target server.
4. Forward the request, minus the hop by hop headers.
5. Change the GET request.
6. Add the ‘X-Forwarded-For’ header.
7. Read the response from the target server and set the correct response headers before,
8. Sending the response to the client.
*/

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const ADDR: &str = "127.0.0.1:8989";

fn main() {
    let listener = TcpListener::bind(ADDR).unwrap();
    println!("Starting proxy server on {}", ADDR);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => println!("{}", e)
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let client_addr = stream.peer_addr().unwrap();

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let (host, _port) = parse_request(http_request);
    println!("Request made. Target: {} Client: {:?}", host, client_addr);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

fn parse_request(http_request: Vec<String>) -> (String, u16) {
    let request_line = &http_request[0]; // "GET http://httpbin.org/ip HTTP/1.1"
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let url = parts[1]; // This gives us "http://httpbin.org/ip"
    let host_and_path = url.trim_start_matches("http://").trim_start_matches("https://"); 
    let parts: Vec<&str> = host_and_path.split('/').collect();
    let host = parts[0]; // This gives us "httpbin.org"
    let host_parts: Vec<&str> = host.split(':').collect();
    let host_name = host_parts[0]; // This gives us "httpbin.org"
    let port = if host_parts.len() == 2 {
        host_parts[1].parse::<u16>().unwrap() // Parse the port if explicitly stated
    } else {
        if url.starts_with("https://") { 443 } else { 80 } // Default ports
    };
    return (host_name.to_owned(), port)
}
