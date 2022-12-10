use std::error::Error;
use std::str::Lines;
use utils::{pick_part_to_solve, read_input_file, Part};

fn main() -> Result<(), Box<dyn Error>> {
    // let filename = "test.txt".to_string();
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

    let answer_a = part_a(&mut lines);
    println!("Part A: {}", answer_a);

    Ok(())
}

// much nicer than part a
fn part_b(mut lines: &mut Lines) {
    let instructions = create_instructions(&mut lines);
    let mut line_point = 0;
    let mut sprite_location = 1;
    for inst in instructions {
        match overlaps_sprite(&sprite_location, &line_point) {
            true => print!("#"),
            false => print!("."),
        }
        sprite_location += inst;

        line_point += 1;
        if line_point >= 40 {
            println!("");
            line_point = 0;
        }
    }
}

fn overlaps_sprite(sprite: &i32, pix: &i32) -> bool {
    return *pix - 1 == *sprite || *pix == *sprite || *pix + 1 == *sprite;
}

fn create_instructions(lines: &mut Lines) -> Vec<i32> {
    let mut instructions = Vec::new();
    for line in lines {
        if line[..4] == *"addx" {
            let to_add = line[5..].parse::<i32>().unwrap();
            instructions.push(0);
            instructions.push(to_add);
        } else if line[..4] == *"noop" {
            instructions.push(0);
        } else {
            println!("Unknown instruction: {}", line);
            panic!("Unknown instruction");
        }
    }
    instructions
}

// Ohh boy was this a bad solution...
fn part_a(lines: &mut Lines) -> i32 {
    let mut counter = 1;
    let mut accum = 0;
    let mut cycles = 1;
    while let Some(line) = lines.next() {
        if line[..4] == *"addx" {
            if add_check_cycle(&cycles) {
                accum += counter * round_cycles(&cycles);
                let temp = round_cycles(&cycles);
                println!(
                    "ADD - Adding {} to accum. Cycle: {} -> {}, , X: {}",
                    counter * round_cycles(&cycles),
                    cycles,
                    temp,
                    counter
                );
            }
            let to_add = line[5..].parse::<i32>().unwrap();
            cycles += 2;
            counter += to_add;
        } else if line[..4] == *"noop" {
            if noop_check_cycle(&cycles) {
                println!(
                    "NOOP - Adding {} to accum. Cycle: {}, X: {}",
                    counter * round_cycles(&cycles),
                    cycles,
                    counter
                );
                accum += counter * cycles;
            }
            cycles += 1;
        } else {
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
    (cycles + 20) % 40 == 0
}

fn add_check_cycle(cycles: &i32) -> bool {
    let temp = (cycles + 20) % 40;
    temp == 0 || temp == 39
}
