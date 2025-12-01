use std::io::{self, Read};

fn main() {
    //read entire stdin as a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut pos: i32 = 50; //starting position
    let mut count_zero = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        //format example: "L68" or "R14"
        let dir = line.chars().next().unwrap();
        let dist: i32 = line[1..].parse().unwrap();

        match dir {
            'L' => {
                pos = (pos - dist).rem_euclid(100);
            }
            'R' => {
                pos = (pos + dist).rem_euclid(100);
            }
            _ => panic!("Invalid direction"),
        }

        if pos == 0 {
            count_zero += 1;
        }
    }

    println!("{}", count_zero);
}
