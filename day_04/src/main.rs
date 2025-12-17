#![warn(clippy::all, clippy::nursery)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// https://adventofcode.com/2025/day/4
fn main() {
    // read the input file into a 2d vec of chars
    let f = File::open("./src/input.txt").expect("error opening input file");
    let reader = BufReader::new(f);
    let mut input: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    // the input is square but whatever
    let w: usize = input[0].len();
    let h: usize = input.len();

    let mut total_removable_rolls = 0;

    let mut rolls_removed = 1;

    // remove accessible rolls until no further rolls can be removed
    while rolls_removed != 0 {
        rolls_removed = 0;
        // for every position in the grid
        for y in 0..h {
            for x in 0..w {
                // if the position is blank, skip it
                if input[y][x] == '.' || input[y][x] == 'x' {
                    continue;
                }

                // number of neighbors
                let mut neighbors = 0;

                // calculating positions of neighboring cells (need to make this cleaner)
                let offsets: [(Option<usize>, Option<usize>); 8] = [
                    (usize::checked_sub(y, 1), usize::checked_sub(x, 1)),
                    (usize::checked_sub(y, 1), Some(x)),
                    (usize::checked_sub(y, 1), Some(x + 1)),
                    (Some(y), usize::checked_sub(x, 1)),
                    (Some(y), Some(x + 1)),
                    (Some(y + 1), usize::checked_sub(x, 1)),
                    (Some(y + 1), Some(x)),
                    (Some(y + 1), Some(x + 1)),
                ];

                for offset in offsets {
                    // if offset is out of bounds, skip it
                    if offset.0.unwrap_or(0) >= h || offset.1.unwrap_or(0) >= w {
                        continue;
                    }

                    // check if the offsets are valid, else skip
                    let offset = (
                        match offset.0 {
                            Some(x) => x,
                            None => continue,
                        },
                        match offset.1 {
                            Some(x) => x,
                            None => continue,
                        },
                    );

                    // check if neighbor is a roll
                    if input[offset.0][offset.1] == '@' {
                        neighbors += 1;
                    }
                }

                // a roll can only be removed if it has less than 3 neighbors
                if neighbors < 4 {
                    input[y][x] = 'x';
                    rolls_removed += 1;
                }
            }
        }

        // if no rolls were removed, stop.
        if rolls_removed == 0 {
            break;
        }

        // add the rolls removed this iteration to the total number of rolls removed
        total_removable_rolls += rolls_removed;
    }

    println!("{} rolls can be removed", total_removable_rolls);
}
