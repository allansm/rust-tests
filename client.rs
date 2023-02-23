use std::net::*;
use std::io::*;

fn main() {
    let mut stream = TcpStream::connect("0.0.0.0:7878")
        .unwrap();

    let mut text = String::new();

    println!("text: ");
    std::io::stdin().read_line(&mut text).unwrap();
    
    stream.write_all(text.as_bytes())
        .unwrap();
}
