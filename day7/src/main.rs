use std::cmp;
use std::error::Error;
use std::str::Lines;
use utils::{pick_part_to_solve, read_input_file, Part};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "input.txt".to_string();
    let input = read_input_file(filename)?;
    let mut lines = input.lines();
    let mut answer = 0;

    let part = pick_part_to_solve()?;
    if part == Part::A {
        traverse_a(&mut lines, &mut answer);
    } else if part == Part::B {
        answer = 70000000;
        traverse_b(&mut lines, &mut answer);
    } else {
        Err("Please select a part [A/B]")?;
    }
    println!("Answer: {}", answer);

    Ok(())
}

fn traverse_a(lines: &mut Lines, grand_total: &mut i32) -> i32 {
    let mut accum = 0;
    while let Some(line) = lines.next() {
        if line == "$ cd .." {
            break;
        } else if line[..4] == *"$ cd" {
            accum += traverse_a(lines, grand_total);
        } else if line[..4] == *"$ ls" {
        } else if line[..4] == *"dir " {
            continue;
        } else {
            let filesize = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            accum += filesize;
        }
    }
    if accum <= 100000 {
        *grand_total += accum;
    }
    return accum;
}

fn traverse_b(lines: &mut Lines, smallest: &mut i32) -> i32 {
    let mut accum = 0;
    while let Some(line) = lines.next() {
        if line == "$ cd .." {
            break;
        } else if line[..4] == *"$ cd" {
            accum += traverse_b(lines, smallest);
        } else if line[..4] == *"$ ls" {
        } else if line[..4] == *"dir " {
            continue;
        } else {
            let filesize = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            accum += filesize;
        }
    }
    if 30000000 <= (70000000 - 43956976) + accum {
        *smallest = cmp::min(*smallest, accum);
    }
    return accum;
}
// used space = 43956976
