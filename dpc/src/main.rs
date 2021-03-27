use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, BufWriter, Write};
// use std::thread;
use std::convert::AsRef;
use regex::Regex;

fn handle_client(stream: TcpStream) {

    let mut write = BufWriter::new(stream.try_clone().unwrap());
    let mut read = BufReader::new(stream.try_clone().unwrap());

    let mut data = String::new();

    let exp = Regex::new(r"^id\d+\n$").unwrap();
    
    while read.read_line(&mut data).expect("something fail") != 0 {

        if data.eq("connection_request\n") {
            println!("someone is trying to connect!");

            write.write(b"id123\n").unwrap();
        } else if exp.is_match(data.as_ref()) {

            // Remove the "id" and "\n"
            let id = &data[2..data.len()-1];

            let value: u32 = id.parse().expect("Id is nnot a number");
            println!("His id is now :{:?}", value);
        }
        else {
            println!("Closing connection");
            break;
        }
        data.clear();
   }
   drop(stream);
   drop(write);
   drop(read);

}

fn main() {


    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // thread::spawn(move || {
                    handle_client(stream);
                // });
            },
            Err(e) => println!("Error: {}", e),
        }
    }
    drop(listener);
   
}
