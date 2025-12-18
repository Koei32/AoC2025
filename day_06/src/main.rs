#![warn(clippy::all, clippy::nursery)]

use std::fs::File;
use std::io::{BufRead, BufReader};

/// https://adventofcode.com/2025/day/6
fn main() {
    // read the file lines into a vec
    let f = File::open("./src/input.txt").expect("error opening input file");
    let reader = BufReader::new(f);
    let mut input: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    // extract the operators from the input
    let binding = input.pop().unwrap();
    let operators: Vec<&str> = binding.split_whitespace().collect();

    // solution for part 1
    /*let numbers: Vec<Vec<u64>> = input
        .iter()
        .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut grand_total = 0;

    for i in 0..operators.len() {
        let mut result = 0;
        for j in &numbers {
            match operators[i] {
                "+" => result += j[i],
                "*" => result = if result == 0 { j[i] } else { j[i] * result },
                _ => (),
            };
        }
        grand_total += result;
    }
    println!("the grand total is {}", grand_total);*/

    // solution for part 2

    let mut grand_total: i64 = 0;

    // making a 2d vec of chars out of the input
    let matrix: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

    // transposing `matrix` into a Vec<String> of columns
    let mut columns: Vec<String> = Vec::new();
    for i in 0..matrix[0].len() {
        let mut column: String = String::new();
        for j in &matrix {
            column.push(j[i]);
        }
        columns.push(column.trim().to_string());
    }

    // splitting the columns by empty columns to obtain the values for a single problem
    let chunks: Vec<&[String]> = columns.split(|x| x.is_empty()).collect();
    for column in 0..chunks.len() {
        // this columns result
        let mut result: i64 = 0;
        for x in chunks[column] {
            let num: i64 = x.trim().parse().unwrap();
            match operators[column] {
                "+" => result += num,
                "*" => result = if result == 0 { num } else { result * num },
                _ => panic!("operator missing"),
            };
        }
        grand_total += result;
    }

    println!("the grand total is {}", grand_total);
}
