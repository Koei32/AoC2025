#![warn(clippy::all, clippy::nursery)]

use std::{
    fs::File,
    io::{BufReader, Read},
};

/// https://adventofcode.com/2025/day/2
fn main() {
    // reads input file into a string
    let f = File::open("./src/input.txt").expect("error opening input file");
    let mut reader = BufReader::new(f);
    let mut inputfile = String::new();
    reader
        .read_to_string(&mut inputfile)
        .expect("error reading buffer");

    // turns "a-b,c-d,e-f,..." into [(a, b), (c, d), (e, f), ...]
    let inputs: Vec<(u64, u64)> = inputfile
        .split(",")
        .map(|x| x.split_once("-").expect("error splitting"))
        .map(|x| {
            (
                x.0.parse().expect("parsing error"),
                x.1.parse().expect("parsing error"),
            )
        })
        .collect();

    // sum of all invalid ids
    let mut sum: u64 = 0;

    for input in inputs {
        let mut id: String;
        // iterating over the range
        for i in input.0..input.1 + 1 {
            id = i.to_string(); // this line may hurt performance, im not sure
            let id_len: usize = id.len();

            // splitting the id in parts of size `substr_len`
            for substr_len in 1..(id_len / 2) + 1 {
                // if the id cannot be split cleanly, we advance to next substr_len
                if !(id_len).is_multiple_of(substr_len) {
                    continue;
                }

                // vec that will store the splits of the id
                let mut splits: Vec<&str> = Vec::new();

                // building the `splits` vec
                for split_pos in (0..id_len).step_by(substr_len) {
                    splits.push(&id[split_pos..split_pos + substr_len]);
                }

                // if we are left with only one element in `splits` after `dedup`ing it, we know
                // that the id was invalid as it would be made up by repeating the subtstring
                // remaining in the vec.
                splits.dedup();
                if splits.len() == 1 {
                    sum += i;
                    break;
                }
            }
        }
    }

    // i am not particularly happy with this solution. i don't enjoy the nesting going on.
    // i'm sure there is a better more clever way to achieve this but i could not figure it out.

    println!("sum is {}", sum);
}
