use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // 绑定到本地地址和端口
    println!("Server is running on 127.0.0.1:7878");

   
    for stream in listener.incoming() {
    let  mut stream = stream.unwrap();
        println!("Connection established!");

        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
 