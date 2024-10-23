use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut steam = TcpStream::connect("127.0.0.1:7878").unwrap();
    steam.write("hello".as_bytes()).unwrap(); // 原始字节

    let mut buffer = [0; 5]; // 创建一个5字节的缓冲区
    steam.read(&mut buffer).unwrap();
    println!("Response from server: {:?}", str::from_utf8(&buffer).unwrap());
}
