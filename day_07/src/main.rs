#![warn(clippy::all, clippy::nursery)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// https://adventofcode.com/2025/day/7
fn main() {
    let f = File::open("./src/input.txt").expect("error opening input file");
    let reader = BufReader::new(f);
    let mut input: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.chars().collect())
        .collect();

    // solution for part 1

    // number of times the beam is split
    let mut split_count: i32 = 0;

    // for every position in the manifold
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            match input[y][x] {
                // if it is already a beam, continue.
                '|' => continue,

                // if it is empty space (denoted by '.'), turn it into a beam if the position
                // above it is also a beam
                '.' => {
                    if input[if y == 0 { 0 } else { y - 1 }][x] == '|' {
                        input[y][x] = '|';
                    }
                }

                // if it is the source of the beam (denoted by 'S'), make the position below
                // it a beam
                'S' => input[y + 1][x] = '|',

                // if it is a splitter (denoted by '^'), and there is a beam above it,
                // turn the spaces adjacent to it into beams
                '^' => {
                    if input[y - 1][x] == '|' {
                        input[y][x - 1] = '|';
                        input[y][x + 1] = '|';
                        split_count += 1;
                    }
                }
                _ => (),
            }
        }
    }
    println!("tachyon split {} times", split_count);

    // solution for part 2

    // total number of timelines that exist
    let mut timeline_count: i64 = 0;

    // memoization hashmap to store the number of timelines from a particular position
    let mut memo: HashMap<(usize, usize), i64> = HashMap::new();

    // calculate the number of paths for every position at the bottom of the manifold where
    // a tachyon beam has exited
    for i in 0..input[0].len() {
        if input[input.len() - 1][i] == '|' {
            timeline_count += calculate_num_paths(input.len() - 1, i, &input, &mut memo);
        }
    }

    println!("number of timelines is {}", timeline_count);
}

/// Recursively calculates and returns the number of paths that can be taken to reach the position
/// `(y, x)` in the `manifold`. Takes in a `memo` HashMap to store previously calculated number of
/// paths for particular postions to be able to lookup later for calculation of other positions.
fn calculate_num_paths(
    y: usize,
    x: usize,
    manifold: &Vec<Vec<char>>,
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    // if the position exists in the memo, then return the value
    if memo.contains_key(&(y, x)) {
        return *memo.get(&(y, x)).unwrap();
    }

    // number of paths to this postion from source
    let mut num_paths: i64 = 0;

    // we start from the bottom and work our way up retracing the path of the beam.

    // checks the position above this position
    match manifold[y - 1][x] {
        // if it is a beam...
        '|' => {
            // ...check if the neighbors are splitters

            // if splitter on left
            if x != 0 && manifold[y - 1][x - 1] == '^' {
                // recursively call this function to calculate the number of paths to this position
                let result = calculate_num_paths(y - 1, x - 1, manifold, memo);
                num_paths += result;

                // insert the result for this position into the memo
                memo.insert((y, x), result + num_paths);
            }

            // if splitter on right
            if x <= manifold[0].len() - 2 && manifold[y - 1][x + 1] == '^' {
                // recursively call this function to calculate the number of paths to this position
                let result = calculate_num_paths(y - 1, x + 1, manifold, memo);
                num_paths += result;

                // insert the result for this position into the memo
                memo.insert((y, x), result + num_paths);
            }

            // recursion
            let result = num_paths + calculate_num_paths(y - 1, x, manifold, memo);

            // insert the result for this position into the memo and return the result
            memo.insert((y, x), result);
            result
        }

        // if we reach the source, we have traced one complete path.
        'S' => {
            memo.insert((y, x), num_paths + 1);
            num_paths + 1
        }

        // we don't care about any other symbol
        _ => {
            memo.insert((y, x), num_paths);
            num_paths
        }
    }
}
