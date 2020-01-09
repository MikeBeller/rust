use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    stream.read(&mut buf)?;
    let reqstr = String::from_utf8_lossy(&buf[..]);
    let rs2 = reqstr.trim_matches(char::from(0));
    let response = format!("HTTP/1.1 200 OK\r\n\r\nReceived: {} Bytes\r\n", rs2.len());
    stream.write(response.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?).unwrap();
    }
    Ok(())
}
