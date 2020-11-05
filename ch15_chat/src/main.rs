use std::fs::File;
use std::io::{self, Read, BufRead};

struct ChatResponse {
    key: String,
    response: String
}

fn main() -> io::Result<()> {
    let filename = "chatresponses.txt";
    let mut response_vector = vec![];

    let file = File::open(filename).expect("unable to open file");
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line.unwrap();
        let mut split_line = line.as_str().split('\t');
        let r = ChatResponse { 
            key: String::from(split_line.next().unwrap()),
            response: String::from(split_line.next().unwrap())
        };
        response_vector.push(r);
    }

    println!("Hi,my name is Zelia, what can I do for you?");
    loop {
        let mut query_input = String::new();
        let mut found: bool = false;
        match io::stdin().read_line(&mut query_input) {
            Ok(_) => query_input = query_input.trim().to_string(),
            Err(error) => panic!("Error: {}", error),
        }
        for resp in &response_vector {
            if query_input.contains(resp.key.as_str()) {
                found = true;
                println!("{}", resp.response);
                break;
            }
        }
        if !found {
            println!("I'm not sure what you are saying");
        }
    }
}