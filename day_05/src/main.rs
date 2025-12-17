#![warn(clippy::all, clippy::nursery)]

use std::{
    cmp::{max, min},
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
};

/// https://adventofcode.com/2025/day/5
fn main() {
    let f = File::open("./src/input.txt").expect("error opening input file");
    let mut reader = BufReader::new(f);
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .expect("error reading from buffer");

    // split the input into ranges and available ids
    let split: Vec<Vec<&str>> = input
        .split("\r\n\r\n")
        .map(|x| x.split_whitespace().collect())
        .collect();

    let mut id_ranges: Vec<(i64, i64)> = split[0]
        .iter()
        .map(|x| x.split_once("-").unwrap())
        .map(|x| (x.0.parse().unwrap(), x.1.parse().unwrap()))
        .collect();

    //  solution for part 1
    /* let mut found_ids: HashSet<i64> = HashSet::new();
    let available_ids: Vec<i64> = split[1].iter().map(|x| x.parse().unwrap()).collect();
    for range in id_ranges {
        for id in &available_ids {
            if range.0 < *id && *id < range.1 {
                found_ids.insert(*id);
            }
        }
    }
    println!("{} valid ids", found_ids.len()); */

    let mut range_set: HashSet<(i64, i64)> = HashSet::new();

    // keeping track of
    let mut last_range_size = range_set.len();

    // loop until all possible range merges are done
    loop {
        // compare every range to every other range in the list
        for sel in 0..id_ranges.len() {
            for cmp in 0..id_ranges.len() {
                // if we are comparing to ourselves, skip.
                if id_ranges[sel] == id_ranges[cmp] {
                    continue;
                }

                // if the ranges intersect, we merge them.
                if id_ranges[cmp].0 <= id_ranges[sel].1 && id_ranges[sel].0 <= id_ranges[cmp].1 {
                    // start is the minimum of both ranges and end is the max of both ranges
                    id_ranges[sel] = (
                        min(id_ranges[sel].0, id_ranges[cmp].0),
                        max(id_ranges[sel].1, id_ranges[cmp].1),
                    );

                    // making both the selected range and compared range be the merged range
                    // (this will result in the merged range appearing twice in the vec)
                    id_ranges[cmp] = id_ranges[sel];
                }
            }
        }
        // making a HashSet from `id_ranges` to eliminate duplicates
        range_set = id_ranges.clone().into_iter().collect();

        // if no new range merges were made, break.
        if range_set.len() == last_range_size {
            break;
        }

        last_range_size = range_set.len();
    }

    let mut num_valid_ids = 0;

    // calculating number of valid ids from the ranges
    for i in range_set {
        num_valid_ids += i.1 - i.0 + 1;
    }

    println!("{} valid ids exist.", num_valid_ids)
}
