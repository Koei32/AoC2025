#![warn(clippy::all, clippy::nursery)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// https://adventofcode.com/2025/day/1
fn main() {
    // reads input file into a buffer
    let f = File::open("./src/input.txt").expect("error opening input file");
    let reader = BufReader::new(f);

    // split inputs by newlines
    let inputs: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    // dial starts at 50
    let mut dial: i32 = 50;

    // password is number of times the dial is at zero
    let mut pwd: i32 = 0;

    for input in inputs {
        // number of clicks
        let clicks: i32 = input.get(1..).unwrap().parse::<i32>().unwrap();

        // direction of rotation (1 is right, -1 is left)
        let direction: i32 = match input.get(0..1) {
            Some("R") => 1,
            Some("L") => -1,
            _ => 0,
        };

        // target after this rotation
        let next: i32 = clicks + (direction * dial);

        // rotating the dial and checking every click (may not be the most optimal technique)
        while dial as i32 != next as i32 {
            dial += direction;
            if dial % 100 == 0 {
                pwd += 1;
            }
        }
    }
    print!("\nthe password is {}", pwd);
}
