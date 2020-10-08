use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

fn make_directory(param: &str)  -> String {
    match fs::create_dir_all(param) {
        Ok(_) =>String::from("Success"),
        Err(err) => err.to_string()
    }
}

fn get_file_list() -> String {
    let mut listing = String::with_capacity(8192);

    for file in fs::read_dir(".").unwrap() {
        let entry = file.unwrap().path().display().to_string();
        listing.push_str(entry.as_str());
    }
    listing
}

fn handle_req(mut conn: TcpStream) {
    let mut reqbytes = [0; 512];
    let mut response = String::with_capacity(8192);

    match conn.write(b"> ")  {
        Ok(_) => (),
        Err(err) => println!("Received an error on write! {}", err)
    };
    let requestsize = conn.read(&mut reqbytes);
    let req = String::from_utf8_lossy(&reqbytes);
    let size = requestsize.unwrap();
    let mut request: String = String::from_utf8(reqbytes[..size].to_vec()).unwrap();
    if size > 0 {
        println!("Received: {}", request);
        let mut params = request.split_whitespace();
        let command = params.next().unwrap();
        match command {
            "flist" => response = get_file_list(),
            "md" => response = make_directory(params.next().unwrap()),
             _ => response = String::from("Unacceptable command")
        }
        match conn.write(&response.as_bytes()) {
            Ok(_) => (),
            Err(err) => println!("Received an error on write! {}", err)
        };
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3333")?;

    for stream in listener.incoming() {
        handle_req(stream?);
    }

    Ok(())
}
