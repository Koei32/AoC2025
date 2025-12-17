#![warn(clippy::all, clippy::nursery)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// https://adventofcode.com/2025/day/3
fn main() {
    // read input file into a Vec<Vec<u32>>
    let f = File::open("./src/input.txt").expect("error opening input file");
    let reader = BufReader::new(f);
    let inputs: Vec<Vec<u32>> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.chars().map(|y| y.to_string().parse().unwrap()).collect())
        .collect();

    let mut total_joltage: u64 = 0;

    for mut input in inputs {
        let mut joltage: String = String::new(); // output joltage for this bank of batteries
        let mut last_max_pos = 0; // position of the last maximum joltage
        let mut remaining_batteries = 12; // number of batteries that need to be turned on

        // loop until we've turned on 12 batteries
        while remaining_batteries != 0 {
            // finding maximum joltage battery in the entire bank excluding the
            // last `remaining_batteries - 1`
            let max = *input[last_max_pos..(input.len() - remaining_batteries + 1)]
                .iter()
                .rev() // because `.max()` finds the last occurence of the max element but we want the first
                .max()
                .unwrap();

            // adding `max` to this bank's joltage as a digit
            joltage.push_str(&max.to_string());

            // i really hate the way last_max_pos is calculated but this is the best i could come up with
            last_max_pos = input[last_max_pos..input.len() - remaining_batteries + 1]
                .iter()
                .position(|x| *x == max)
                .unwrap()
                + last_max_pos;

            // removing the max battery from the bank
            input.remove(last_max_pos);

            remaining_batteries -= 1;
        }
        // add this bank's joltage to the total joltage
        total_joltage += joltage.parse::<u64>().expect("error parsing");
    }

    println!("total joltage is {}", total_joltage);
}
