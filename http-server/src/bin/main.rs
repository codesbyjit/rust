use http_server::ThreadPool;
use std::collections::btree_map::Range;
use std::fs;
use std::hash::RandomState;
use std::io::Result;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream?;
        println!("Connected: {:?}", stream.peer_addr()?);
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting Down");

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_next = b"GET /next HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200", "index.html")
    } else if buffer.starts_with(get_next) {
        thread::sleep(Duration::from_secs(5)); // for test
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
