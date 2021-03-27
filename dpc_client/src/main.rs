use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::Command;
use regex::Regex;

fn main() {
    let exp = Regex::new(r"^\d{1}-\w+$").unwrap();

    println!("{}",exp.is_match("1-aabzea"));

    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    let ask_for_connection = b"connection_request\n";
    let id_is = b"id132\n";

    stream.write(ask_for_connection).unwrap();
    stream.write(id_is).unwrap();
    stream.write(b"end").unwrap();

    stream.write(id_is).unwrap();
    let mut data = String::new();
    let mut read = BufReader::new(stream.try_clone().unwrap());



    while read.read_line(&mut data).expect("something fail") != 0 {
        println!("Server said: {}",data);
        if exp.is_match(&data) {
            process_command(&data);
        }
        data.clear();
    }



}

fn process_command(arg: &str) {
    let ip = "192.168.1.51:2377";

    let output = Command::new("sh")
            .arg("-c")
            .arg("docker swarm join --token")
            .arg(arg)
            .arg(ip)
            .spawn()
            .expect("failed to execute process");

    println!("{:?}", output);
}