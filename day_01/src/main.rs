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
    let mut dial: f32 = 50.0;

    // password is number of times the dial is at zero
    let mut pwd: f32 = 0.0;

    for input in inputs {
        // number of clicks
        let clicks: f32 = input.get(1..).unwrap().parse::<f32>().unwrap();

        // direction of rotation (1 is right, -1 is left)
        let direction: f32 = match input.get(0..1) {
            Some("R") => 1.0,
            Some("L") => -1.0,
            _ => 0.0,
        };

        // target after this rotation
        let next: f32 = dial + (clicks * direction);

        // rotating the dial and checking every click (may not be the most optimal technique)
        while dial != next {
            dial += direction;
            if dial % 100.0 == 0.0 {
                pwd += 1.0;
            }
        }
    }
    print!("\nthe password is {}", pwd);
}
