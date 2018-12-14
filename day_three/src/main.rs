extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, Hash, Clone, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Rect {
    origin: Point,
    width: u32,
    height: u32,
}

impl Rect {
    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for x_off in 0..self.width {
            for y_off in 0..self.height {
                points.push(Point {
                    x: self.origin.x + x_off,
                    y: self.origin.y + y_off,
                });
            }
        }
        points
    }
}

fn part_one() -> HashMap<Point, u32> {
    let mut fabric: HashMap<Point, u32> = HashMap::new();
    let file = BufReader::new(File::open("input/input.txt").unwrap());
    let re: regex::Regex =
        regex::Regex::new("#[0-9]+ @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    for line in file.lines() {
        let text = line.unwrap();
        let caps = re.captures(&text).unwrap();
        let rect = Rect {
            origin: Point {
                x: String::from(caps.get(1).unwrap().as_str()).parse().unwrap(),
                y: String::from(caps.get(2).unwrap().as_str()).parse().unwrap(),
            },
            width: String::from(caps.get(3).unwrap().as_str()).parse().unwrap(),
            height: String::from(caps.get(4).unwrap().as_str()).parse().unwrap(),
        };
        let points = rect.points();
        for point in points {
            let fab = match fabric.get_mut(&point) {
                Some(p) => p,
                None => {
                    fabric.insert(point.clone(), 0);
                    fabric.get_mut(&point).unwrap()
                }
            };
            *fab += 1;
        }
    }
    let mut overlap = 0;
    fabric.values().for_each(|x| {
        if *x > 1 {
            overlap += 1;
        }
    });
    println!("{}", overlap);
    fabric
}

fn part_two(fabric: &HashMap<Point, u32>) {
    let file = BufReader::new(File::open("input/input.txt").unwrap());
    let re: regex::Regex =
        regex::Regex::new("#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    for line in file.lines() {
        let text = line.unwrap();
        let caps = re.captures(&text).unwrap();
        let id: u32 = String::from(caps.get(1).unwrap().as_str()).parse().unwrap();
        let rect = Rect {
            origin: Point {
                x: String::from(caps.get(2).unwrap().as_str()).parse().unwrap(),
                y: String::from(caps.get(3).unwrap().as_str()).parse().unwrap(),
            },
            width: String::from(caps.get(4).unwrap().as_str()).parse().unwrap(),
            height: String::from(caps.get(5).unwrap().as_str()).parse().unwrap(),
        };
        let points = rect.points();
        let mut no_overlap = true;
        for point in points {
            let fab = fabric.get(&point).unwrap();
            if *fab > 1 {
                no_overlap = false;
                break;
            }
        }
        if no_overlap {
            println!("{}", id);
        }
    }
}

fn main() {
    let fabric = part_one();
    part_two(&fabric);
}
