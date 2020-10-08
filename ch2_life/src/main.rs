extern crate rand;
extern crate termion;
use std::{env, thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::clear;
use termion::color;

fn main() {
    let mut world = [[0u8; 75]; 75];
    let mut generations = 0;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        for i in 0..74 {
            for j in 0..74 {
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename);
    }

    println!("Population at generation {} is {}", generations, census(world));
    for _gens in 0..100 {
        let temp = generation(world);
        world = temp;
        generations += 1;
        println!("{}", clear::All);
        displayworld(world);
        println!("{blue}Population at generation {g} is {c}", blue = color::Fg(color::Blue), g = generations, c = census(world));
        thread::sleep(time::Duration::from_secs(2));
    }

}

fn populate_from_file(filename: String) -> [[u8; 75]; 75] 
{
    let mut newworld = [[0u8; 75]; 75];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut pairs:  Vec<(usize, usize)> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let mut words = l.split_whitespace();
        let left = words.next().unwrap();
        let right = words.next().unwrap();
        pairs.push((left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap()));
    }

    for i in 0..74 {
        for j in 0..74 {
            newworld[i][j] = 0;
        }
    }

    for (x,y) in pairs {
        newworld[x][y] = 1;
    }
    newworld
}

fn displayworld(world: [[u8; 75]; 75])
{
    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 
            {
                print!("{red}*", red = color::Fg(color::Red));
            }
            else 
            {
                print!(" ");
            }
        }
        println!("");
    }
}

fn census(world: [[u8; 75]; 75]) -> u16
{
    let mut count = 0;

    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1
            {
                count += 1;
            }
        }
    }
    count
}
fn generation(world: [[u8; 75]; 75]) -> [[u8; 75]; 75]
{
    let mut newworld = [[0u8; 75]; 75];

    for i in 0..74 {
        for j in 0..74 {
            let mut count = 0;
            if i>0 {
                count = count + world[i-1][j];
            }
            if i>0 && j>0 {
                count = count + world[i-1][j-1];
            }
            if i>0 && j<74 {
                count = count + world[i-1][j+1];
            }
            if i<74 && j>0 {
                count = count + world[i+1][j-1]
            }
            if i<74 {
                count = count + world[i+1][j];
            }
            if i<74 && j<74 {
                count = count + world[i+1][j+1];
            }
            if j>0 {
                count = count + world[i][j-1];
            }
            if j<74 {
                count = count + world[i][j+1];
            }

            newworld[i][j] = 0;

            if (count <2) && (world[i][j] == 1) {
                newworld[i][j] = 0;
            }
            if world[i][j] == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            }
            if (world[i][j] == 0) && (count == 3) {
                newworld[i][j] = 1;
            }
        }
    }
    newworld
}


