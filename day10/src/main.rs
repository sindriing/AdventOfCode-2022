// use utils::{pick_part_to_solve, read_input_file, Part};
use utils::read_input_file;
use std::error::Error;
use std::str::Lines;

fn main() -> Result<(), Box<dyn Error>> {
    // let filename = "test.txt".to_string();
    let filename = "input.txt".to_string();
    let input = read_input_file(filename)?;
    let mut lines = input.lines();

    let answer_a = part_a(&mut lines);
    println!("Part A: {}", answer_a);

    Ok(())
}

fn part_a(lines: &mut Lines) -> i32 {
    let mut counter = 1;
    let mut accum = 0;
    let mut cycles = 1;
    while let Some(line) = lines.next() {
        if line[..4] == *"addx" {
            if add_check_cycle(&cycles){
                accum += counter * round_cycles(&cycles);
                let temp = round_cycles(&cycles);
                println!("ADD - Adding {} to accum. Cycle: {} -> {}, , X: {}", counter * round_cycles(&cycles), cycles, temp, counter);
            }
            let to_add = line[5..].parse::<i32>().unwrap();
            cycles += 2;
            counter += to_add;
        } else if line[..4] == *"noop" {
            if noop_check_cycle(&cycles) {
                println!("NOOP - Adding {} to accum. Cycle: {}, X: {}", counter * round_cycles(&cycles), cycles, counter);
                accum += counter * cycles;
            }
            cycles += 1;
        }
        else {
            panic!("Unknown instruction");
        }
    }
    accum
}

fn round_cycles(cycles: &i32) -> i32 {
    let mut temp = *cycles;
    if (cycles + 20) % 40 != 0 {
        temp += 1;
    }
    temp
}

fn noop_check_cycle(cycles: &i32) -> bool {
    // 60 -> 61
    // 20 -> 21
    // println!("cycles: {}, mod: {}", cycles, (cycles +20) % 40);
    (cycles + 20) % 40 == 0
}

fn add_check_cycle(cycles: &i32) -> bool {
    // 19 -> 21
    // 59 -> 61 
    // 60 -> 62
    // println!("cycles: {}, mod: {}", cycles, (cycles + 20) % 40);
    let temp = (cycles + 20) % 40;
    temp == 0 || temp == 39
}

// fn check_cycles(cycles: &i32, addition: i32) -> bool {
//     let before = (cycles - 22) % 40; 
//     let after = (cycles - 22 + addition) % 40;
//     // println!("cycles: {}, before: {}, after: {}", cycles, before, after);
//     return before > after;

// }
