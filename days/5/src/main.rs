use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Debug)]
enum Case {
    Upper,
    Lower,
}

#[derive(Debug)]
struct Unit {
    unit: char,
    case: Case,
}

impl Unit {
    fn react(&self, other: &Unit) -> bool {
        if self.unit == other.unit {
            self.case == other.case
        } else {
            false
        }
    }
}

fn get_case(c: &char) -> Case {
    if c.is_uppercase() {
        Case::Upper
    } else {
        Case::Lower
    }
}

fn parse_input() -> Vec<Unit> {
    let mut ret = Vec::new();
    let file = BufReader::new(File::open("input/input.txt").unwrap());
    for line in file.lines() {
        let text = line.unwrap();
        text.chars().for_each(|c| {
            ret.push(Unit {
                case: get_case(&c),
                unit: c.to_lowercase().to_string().chars().next().unwrap(),
            })
        });
    }
    ret
}

fn scan_polymer(input: Vec<Unit>) -> Vec<Unit> {
    let mut polymer: Vec<Unit> = Vec::with_capacity(input.len());
    println!(
        "scanning, input_len:{}, polymer_len: {}",
        input.len(),
        polymer.len()
    );
    for unit in input {
        println!("scanning polymer_len: {}", polymer.len());
        match polymer.last() {
            Some(u) => {
                if u.react(&unit) {
                    polymer.pop();
                } else {
                    polymer.push(unit);
                }
            }
            None => {
                polymer.push(unit);
            }
        }
    }
    polymer
}

fn main() {
    let input = parse_input();
    let poly = scan_polymer(input);
    println!("{:?}, {}", poly, poly.len());
}
