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

fn most_slept_guard(entries: &Vec<Entry>) -> u32 {
    let mut guards: HashMap<u32, u32> = HashMap::new();
    let mut guard = 0;
    let mut sleep = 0;
    for entry in entries {
        match entry.event {
            Event::Begin(g) => guard = g,
            Event::Sleep => sleep = entry.date_time.minute,
            Event::Wake => {
                match guards.get_mut(&guard) {
                    Some(g) => *g += entry.date_time.minute - sleep,
                    None => {
                        guards.insert(guard, entry.date_time.minute - sleep);
                    }
                };
            }
        }
    }

    let mut most_slept = 0;
    let mut m_guard = 0;
    guards.iter().for_each(|(k, v)| {
        if *v > most_slept {
            m_guard = *k;
            most_slept = *v;
            println!("{}", most_slept);
        }
    });
    println!("{:?}", guards.values().max().unwrap());
    m_guard
}

fn get_sleep_overlap(entries: &Vec<Entry>, guard: u32) -> u32 {
    let mut sleep_map: HashMap<u32, u32> = HashMap::new();
    let mut curr_guard: u32 = 0;
    let mut sleep = 0;
    for entry in entries {
        match entry.event {
            Event::Begin(g) => curr_guard = g,
            Event::Sleep => sleep = entry.date_time.minute,
            Event::Wake => {
                if curr_guard == guard {
                    println!("{}-{}", sleep, entry.date_time.minute);
                    for min in sleep..entry.date_time.minute {
                        match sleep_map.get_mut(&min) {
                            Some(m) => *m += 1,
                            None => {
                                sleep_map.insert(min, 1);
                            }
                        }
                    }
                };
            }
        }
    }

    let mut most_slept = 0;
    let mut min = 0;
    sleep_map.iter().for_each(|(k, v)| {
        if *v > most_slept {
            most_slept = *v;
            min = *k;
        }
    });
    min
}

fn main() {
    let mut input = parse_input();
    println!("{:?}", input);
    input.sort();
    input.iter().for_each(|x| println!("{:?}", x));
    let most_slept = most_slept_guard(&input);
    println!("{}", most_slept);
    let overlap = get_sleep_overlap(&input, most_slept);
    println!("{}", overlap);
}
