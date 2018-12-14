extern crate regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq)]
struct DateTime {
    month: u32,
    day: u32,
    minute: u32,
}

impl std::cmp::PartialEq for DateTime {
    fn eq(&self, other: &DateTime) -> bool {
        self.month == other.month && self.day == other.day && self.minute == other.minute
    }
}

impl std::cmp::Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> Ordering {
        match self.month.cmp(&other.month) {
            Ordering::Equal => match self.day.cmp(&other.day) {
                Ordering::Equal => self.minute.cmp(&other.minute),
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            },
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl std::cmp::PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq)]
enum Event {
    Begin(u32),
    Sleep,
    Wake,
}

impl std::cmp::PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self == other
    }
}

#[derive(Debug, Eq)]
struct Entry {
    date_time: DateTime,
    event: Event,
}

impl std::cmp::PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.date_time == other.date_time && self.event == other.event
    }
}

impl std::cmp::Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.date_time.cmp(&other.date_time)
    }
}

impl std::cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input() -> Vec<Entry> {
    let mut entries = Vec::new();
    let file = BufReader::new(File::open("input/input.txt").unwrap());
    let re: regex::Regex =
        regex::Regex::new(r"\[\d+-(\d+)-(\d+) \d+:(\d+)\] (\w+) #?(\d*)").unwrap();
    for line in file.lines() {
        let text = line.unwrap();
        let caps = re.captures(&text).unwrap();
        entries.push(Entry {
            date_time: DateTime {
                month: String::from(caps.get(1).unwrap().as_str()).parse().unwrap(),
                day: String::from(caps.get(2).unwrap().as_str()).parse().unwrap(),
                minute: String::from(caps.get(3).unwrap().as_str()).parse().unwrap(),
            },
            event: match caps.get(4).unwrap().as_str() {
                "Guard" => {
                    Event::Begin(String::from(caps.get(5).unwrap().as_str()).parse().unwrap())
                }
                "wakes" => Event::Wake,
                "falls" => Event::Sleep,
                _ => panic!(format!("Something broke at line: {:?}", caps)),
            },
        });
    }
    entries
}

fn main() {
    let mut input = parse_input();
    println!("{:?}", input);
    input.sort();
    input.iter().for_each(|x| println!("{:?}", x));
}
