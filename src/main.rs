use std::{io::{Read, Write}, net::{TcpListener, TcpStream}}; 
use std::fs;

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT : &str ="8477";

    let end_point: String = HOST.to_owned() + ":" + PORT;

    let listener = TcpListener::bind(end_point).unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection estabilished");
        handle_connection(_stream);
    }

}

fn handle_connection(mut stream: TcpStream){
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let content = fs::read_to_string("index.html");
    let error = b"HTTP/1.1 500 Page Not Found\r\n\r\n";
 
    match content {
        Ok(content) => {
            let response  = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(_) => {stream.write(error).unwrap();stream.flush().unwrap();}
    }
}