use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();


    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream).unwrap();
    }    
}

fn handle_connection(mut stream: TcpStream) -> Result<(), &'static str> {
    // println!("A");
    // let buf_reader = BufReader::new(&mut stream);

    // println!("{:#?}", buf_reader);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();

    // println!("B");
    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    let contents = fs::read_to_string("hello.html").unwrap();

    let length = contents.len();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
    println!("{response}");

    Ok(())
}
