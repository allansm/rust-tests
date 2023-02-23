use std::net::*;
use std::io::*;

fn main(){
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    println!("waiting...");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        for n in BufReader::new(&mut stream).lines(){
            println!("{}", n.unwrap());
        }
    }
}

