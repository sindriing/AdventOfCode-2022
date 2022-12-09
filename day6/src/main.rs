use utils::{Part, pick_part_to_solve, read_input_file};
use std::error::Error;
// use utils::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "test.txt".to_string();
    let input = read_input_file(filename)?;
    let part = pick_part_to_solve()?;
    let ans;
    match part {
        Part::A => ans = solve_a(&input).unwrap(),
        Part::B => ans = solve_b(&input).unwrap(),
    };
    println!("Solution: {}", ans);

    Ok(())
}

fn solve_a(input: &String) -> Option<i32> {
    let line = input.lines().next().unwrap();
    let mut lastn: Vec<char> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        check_next_signal(c, &mut lastn);
        if lastn.len() >= 4 {
            return Some((i as i32)+1);
        }
    }
    return None;
}

fn solve_b(input: &String) -> Option<i32> {
    let line = input.lines().next().unwrap();
    let mut lastn: Vec<char> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        check_next_signal(c, &mut lastn);
        if lastn.len() >= 14 {
            return Some((i as i32)+1);
        }
    }
    return None;
}

fn check_next_signal(c: char, lastn: &mut Vec<char>) {
    if lastn.contains(&c) {
        while lastn[0] != c {
            lastn.remove(0);
        }
        lastn.remove(0);
    }
    lastn.push(c);
}
