#![allow(unused)]
use std::net::TcpStream;
fn main() {
    use std::net::TcpStream;
    println!("Hello, world!");
    can_be_reached("192.168.1.1",80)
}

fn can_be_reached(ip: &str , port: i32) {
    if let Ok(stream) = TcpStream::connect(ip.to_string() +":"+&port.to_string()) {
    println!("the address has been reached");
} else {
    println!("Couldn't reached the address...");
}
}