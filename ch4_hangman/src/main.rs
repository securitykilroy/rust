extern crate rand;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

struct Word { 
    answer: String,
    length: usize,
    correct_count: usize,
    representation: String
}

trait CheckLetter {
    fn check_for_letter(&mut self, c: char) -> bool;
}

trait CheckComplete {
    fn check_complete(&self) -> bool;
}

impl CheckComplete for Word {
    fn check_complete(&self) -> bool {
        self.correct_count == self.length
    }
}

impl CheckLetter for Word {
    fn check_for_letter(&mut self, c: char) -> bool {
        let mut count: usize = 0;
        let mut found: bool = false;
        let mut response = String::with_capacity(self.length);
        let mut index = 0;
        for letter in self.answer.chars() {
            if letter == c {
                found = true; 
                count += 1;
                response.push(c);
            }
            else {
                if self.representation.chars().nth(index) != Some('_') {
                    response.push(self.representation.chars().nth(index).unwrap());
                }
                else {
                response.push('_');
                }
            }
            index += 1;
        }
        if found {
            println!("Found a ")
        }
        self.representation = response;
        self.correct_count += count;
        count > 0
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_list(filename: String) -> Vec::<String> {
    let mut v = Vec::<String>::new();
    if let Ok(lines) = read_lines(filename) {
        for w in lines { 
            let word:String = w.unwrap();
            if word.len() > 4 {
                v.push(word);
            }
        }
    }
    v
}

fn select_word() -> String {
    let mut rng = rand::thread_rng();
    let filename:String = "words.txt".to_string();
    let words:Vec<String> = read_list(filename);
    let word_count = words.len();
    let selection = rng.gen_range(1, word_count);
    let select: String = words[selection].clone();
    select
}

fn main() {
 
    let body = vec!["noose".to_string(), "head".to_string(), "neck".to_string(), "torso".to_string(), "left arm".to_string(), 
    "right arm".to_string(), "right leg".to_string(), "left leg".to_string(), "left foot".to_string(), "right foot".to_string()];
    let mut body_iter = body.iter();
    let mut result = select_word();
    let mut answer = Word {
        length: result.len(),
        representation: String::from_utf8(vec![b'_'; result.len()]).unwrap(),
        answer: result,
        correct_count: 0
    };

    let mut letter: char;
    let mut body_complete: bool = false;
    while !answer.check_complete() && !body_complete {
        println!("Provide a letter to guess ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                letter = input.chars().nth(0).unwrap();
                if answer.check_for_letter(letter) {
                    println!("There is at least one {}, so the word is {}", letter, answer.representation);
                }
                else {
                    let next_part = body_iter.next().unwrap();
                    println!("Incorrect! You are at {}", next_part);
                    if next_part == "right foot" {
                        body_complete = true;
                    }
                }
            }
            Err(error) => {
                println!("Didn't get any input");
            }
        }
    }
    if body_complete {
        println!("You were unsuccessful at guessing {}", &answer.answer)
    }
    else {
        println!("Yes! The word was {}", &answer.answer);
    }
}
