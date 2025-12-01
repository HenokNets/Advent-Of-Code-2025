use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");
    
    let result = solve(&input);
    println!("Solution: {}", result);
}

fn solve(input: &str) -> String {
    let mut position = 50i32;
    let mut count = 0;
    
    for line in input.lines() {
        if line.len() < 2 {
            continue;
        }
        
        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().unwrap_or(0);
        
        let start = position;
        let end;
        
        if direction == "R" {
            end = (position + distance) % 100;
        } else { // "L"
            end = (position - distance).rem_euclid(100);
        }
        
        //count all times we hit 0 during the rotation
        if direction == "R" {
            //moving right (increasing)
            for i in 1..=distance {
                let intermediate = (start + i) % 100;
                if intermediate == 0 {
                    count += 1;
                }
            }
        } else {
            //moving left (decreasing)
            for i in 1..=distance {
                let intermediate = (start - i).rem_euclid(100);
                if intermediate == 0 {
                    count += 1;
                }
            }
        }
        
        position = end;
    }
    
    count.to_string()
}