use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() {
    let mut freq = 0;
    let file = BufReader::new(File::open("input/input.txt").unwrap());
    for line in file.lines() {
        let mut op = line.unwrap();
        let num_string = op.split_off(1);
        let num = num_string.parse::<i32>().unwrap();
        freq = match op.as_str() {
            "-" => freq - num,
            "+" => freq + num,
            _ => panic!("something broke"),
        };
    }

    println!("freq: {}", freq);
}

fn part_two(seen: &mut HashMap<i32, u8>, freq: &mut i32) -> bool {
    let file = BufReader::new(File::open("input/input.txt").unwrap());
    for line in file.lines() {
        let mut op = line.unwrap();
        let num_string = op.split_off(1);
        let num = num_string.parse::<i32>().unwrap();
        *freq = match op.as_str() {
            "-" => *freq - num,
            "+" => *freq + num,
            _ => panic!("something broke"),
        };
        match seen.get(freq) {
            Some(_) => {
                println!("repeat: {}", freq);
                return true;
            }
            None => {
                seen.insert(*freq, 1);
                ()
            }
        };
    }
    false
}

fn main() {
    part_one();
    let mut seen = HashMap::<i32, u8>::new();
    let mut freq = 0;
    loop {
        if part_two(&mut seen, &mut freq) {
            break;
        }
        println!("freq: {}", freq);
    }
}
