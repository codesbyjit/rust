use std::fs;
use std::io::Result;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;
        println!("Connected: {:?}", stream.peer_addr()?);

        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_next = b"GET /next HTTP/1.1\r\n";

    let(status_line, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200", "index.html")
        } else if buffer.starts_with(get_next) {
            ("HTTP/1.1 200", "next.html")
        } else {
            ("HTTP/1.1 404", "404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

    // println!(
    //     "Request: {}",
    //     String::from_utf8_lossy(&buffer[..])
    // )

    // let contents = fs::read_to_string("index.html").unwrap();
    // let response = format!(
    //     "HTTP/1.1 200 ok\r\nContent-Length: {}\r\n\r\n{}",
    //     contents.len(),
    //     contents
    // );

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();

    // let get = b"GET / HTTP/1.1\r\n";
    // if buffer.starts_with(get) {
    //     let contents = fs::read_to_string("index.html").unwrap();
    //     let response = format!(
    //         "HTTP/1.1 200 ok\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents
    //     );

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let response = format!(
    //         "{}\r\nContent-Length: {}\r\n\r\n{}",
    //         status_line,
    //         contents.len(),
    //         contents
    //     );

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }
// }