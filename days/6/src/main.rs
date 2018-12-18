use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Point {
    coord: Coordinate,
    closest: Coordinate,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        self.coord.distance(&other.coord)
    }
}

fn parse_input() -> Vec<Point> {
    let mut points = Vec::new();
    let file = BufReader::new(File::open("input/input.txt").unwrap());

    for line in file.lines() {
        let text = line.unwrap();
        let split: Vec<&str> = text.split(", ").collect();
        let coord = Coordinate {
            x: String::from(split[0]).parse().unwrap(),
            y: String::from(split[1]).parse().unwrap(),
        };
        points.push(Point {
            closest: coord.clone(),
            coord,
        });
    }

    points
}

fn main() {
    println!("{:?}", parse_input());
}
