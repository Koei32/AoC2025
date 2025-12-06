use std::{fs::read_to_string};

/// https://adventofcode.com/2025/day/1


fn main() {
    // reads input file into a string
    let inputfile: String = read_to_string("./src/input.txt")
        .expect("error opening input file.");

    // split inputs by newlines
    let inputs: Vec<&str> = inputfile.lines().collect();

    // dial starts at 50
    let mut dial: i32 = 50;
    
    // password is number of times the dial is at zero
    let mut pwd: i32 = 0;

    for input in inputs{
        let turn = match input.get(0..1) {
            Some("R") => input.get(1..).unwrap().parse::<i32>().unwrap(),
            Some("L") => input.get(1..).unwrap().parse::<i32>().unwrap() * -1,
            _ => 0
        };

        dial += turn;
        // the dial has numbers 0 to 99, so if the current dial number is divisible
        // by 100, the dial is at zero. this way we dont need to simulate an actual dial and
        // worry about our `dial` number being negative.
        if dial % 100 == 0 {
            pwd += 1;
        }
    }

    print!("the password is {}", pwd);
}
