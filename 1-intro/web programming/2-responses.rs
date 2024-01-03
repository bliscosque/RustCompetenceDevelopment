use std::net::{TcpListener, TcpStream};
use std::io::{BufReader,BufRead, Write};
use std::io::prelude;
use std::fs; 

fn main() {
    let listener=TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream=stream.unwrap();
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader=BufReader::new(&mut stream);
    let http_request=buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|lines| !lines.is_empty())
        .collect::<Vec<String>>();

    println!("Requests: {:#?}",http_request);

    /* 
            Responce Syntax 

            HTTP-Version Status-Code Reason-Phrase CRLF 
            headers CRLF
            message-body 

            ex: HTTP/1.1 200 OK\r\n\r\n
    */
    /*

    // Returning only 200OK (without body)

    let response = "HTTP/1.1 200 OK\r\n\r\n"; 
    stream.write(response.as_bytes()).unwrap(); 
    stream.flush().unwrap(); 
     */

    let status_line = "HTTP/1.1 200 OK \r\n"; 
    let contents = fs::read_to_string("index.html").unwrap();  
    let length = contents.len(); 
}