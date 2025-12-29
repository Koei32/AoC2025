#![warn(clippy::all, clippy::nursery)]

use core::f32;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, Debug)]
struct JuncBox {
    x: i64,
    y: i64,
    z: i64,
}
impl JuncBox {
    fn distance(&self, other: &Self) -> f32 {
        f32::sqrt(
            ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
                as f32,
        )
    }
}
impl FromIterator<i64> for JuncBox {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let mut x = iter.into_iter();
        Self {
            x: x.next().unwrap(),
            y: x.next().unwrap(),
            z: x.next().unwrap(),
        }
    }
}

/// https://adventofcode.com/2025/day/8
fn main() {
    // read the file lines into an input vec
    let f = File::open("./src/input.txt").expect("error opening input file");
    let reader = BufReader::new(f);
    let input: Vec<JuncBox> = reader
        .lines() // split by lines
        .map(|x| x.unwrap())
        .map(|x| {
            x.split(",") // split by commas
                .map(|x| x.parse::<i64>().unwrap()) // parse into i64s
                .collect::<JuncBox>() // collect into `JuncBox`
        })
        .collect();

    // number of connections to make (part 1)
    // let conn_count = 1000;

    // vec that stores which box each box is connected to (adjacency list)
    let mut conn_list: Vec<Vec<usize>> = vec![Vec::new(); input.len()];

    // constructing the distance map
    let mut distances: Vec<((usize, usize), f32)> = Vec::new();
    for this in 0..input.len() {
        for other in (this + 1)..input.len() {
            let dist = input[this].distance(&input[other]);
            distances.push(((this, other), dist));
        }
    }

    // sorting by distances in decreasing order
    distances.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));

    // number of connections we have made (part 1)
    // let mut count = 0;

    for i in distances {
        if conn_list[i.0.0].contains(&i.0.1) {
            continue;
        }

        conn_list[i.0.0].push(i.0.1);
        conn_list[i.0.1].push(i.0.0);

        // part 1
        // count += 1;

        let mut circuits: Vec<Vec<usize>> = Vec::new();
        let mut visited: Vec<bool> = vec![false; input.len()];

        // performing DFS on every unvisited box to group boxes into circuits
        for j in 0..input.len() {
            if !visited[j] {
                let mut circuit: Vec<usize> = Vec::new();
                dfs(&conn_list, &mut visited, j, &mut circuit);
                circuits.push(circuit);
            }
        }

        // (part 1) if we have made the required number of connections, break.
        /*if count == conn_count {
        break;
        }*/

        // (part 2) if all the boxes are part of the same circuit, we have the answer
        if circuits.len() == 1 {
            println!("{} is the answer", input[i.0.0].x * input[i.0.1].x);
            break;
        }
    }

    // solution for part 1
    /* let mut res: Vec<Vec<usize>> = Vec::new();
    let mut visited: Vec<bool> = vec![false; input.len()];

    // performing DFS on every unvisited box
    for j in 0..input.len() {
        if !visited[j] {
            let mut comp: Vec<usize> = Vec::new();
            dfs(&conn_list, &mut visited, j, &mut comp);
            res.push(comp);
        }
    }

    // sort the circuits in decreasing order of their len
    res.sort_by(|a, b| b.len().cmp(&a.len()));

    // selecting the 3 biggest circuits and multiplying them together
    let mut answer: usize = 1;
    for i in res.iter().take(3) {
        answer *= i.len();
    }
    println!("{answer}");*/
}

/// Depth First Search for an undirected graph.
/// recursively searches and marks nodes connected to the passed node.
fn dfs(adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, x: usize, result: &mut Vec<usize>) {
    visited[x] = true;
    result.push(x);

    for i in adj_list[x].clone() {
        if !visited[i] {
            dfs(adj_list, visited, i, result);
        }
    }
}
