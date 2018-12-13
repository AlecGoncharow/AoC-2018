use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() {
    let mut count_map: HashMap<u32, u32> = HashMap::new();
    count_map.insert(2, 0);
    count_map.insert(3, 0);
    let file = BufReader::new(File::open("input/input.txt").unwrap());
    for line in file.lines() {
        let mut char_map: HashMap<char, u32> = HashMap::new();
        let id = line.unwrap();
        id.chars().for_each(|c| match char_map.get_mut(&c) {
            Some(v) => *v += 1,
            None => {
                char_map.insert(c, 1);
            }
        });

        let mut has_two = false;
        let mut has_three = false;
        char_map.values().for_each(|x| match x {
            2 => has_two = true,
            3 => has_three = true,
            _ => (),
        });
        if has_two {
            *count_map.get_mut(&2).unwrap() += 1;
        }
        if has_three {
            *count_map.get_mut(&3).unwrap() += 1;
        }
    }
    println!("{:?}", count_map);
}

fn part_two() {
    let file = BufReader::new(File::open("input/input.txt").unwrap());
    for (i, line) in file.lines().enumerate() {
        let line_clone = line.unwrap().clone();
        let file_same = BufReader::new(File::open("input/input.txt").unwrap());
        let rest_of_lines = file_same.lines().skip(i + 1);

        for other_line in rest_of_lines {
            let other_line_clone = other_line.unwrap().clone();
            let count = other_line_clone
                .chars()
                .zip(line_clone.chars())
                .filter(|&(a, b)| a != b)
                .count();

            if count == 1 {
                println!("{}\n{}", line_clone, other_line_clone);
            }
        }
    }
}

fn main() {
    part_one();
    part_two();
}
