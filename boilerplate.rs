use utils::{pick_part_to_solve, read_input_file, Part};
use std::error::Error;
use std::str::Lines;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "input.txt".to_string();
    let input = read_input_file(filename)?;
    let mut lines = input.lines();
    let part = pick_part_to_solve()?;
    if part == Part::A {
        part_a(&mut lines);
    } else if part == Part::B {
        part_b(&mut lines);
    } else {
        Err("Please select a part [A/B]")?;
    }

    Ok(())
}

fn part_a(mut lines: &mut Lines) {
    println!("Part A:");
}

fn part_b(mut lines: &mut Lines) {
    println!("Part B");
}

