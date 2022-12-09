// use std::{char, error::Error, fs::read_to_string};
use std::{env::args, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let stack_file = "stack.txt".to_owned();
    let raw_stacks = read_input_file(stack_file)?;
    let mut stacks = create_stacks(raw_stacks);

    let moves_file = "moves.txt".to_owned();
    let raw_moves = read_input_file(moves_file)?;
    let crane_moves = get_crane_moves(raw_moves);


    println!("The starting position of the stacks is:");
    print_stacks(&stacks);
    for crane_move in crane_moves {
        let part = pick_part_to_solve()?;
        match part {
            Part::A => perform_crane_move(&mut stacks, &crane_move),
            Part::B => perform_crane_move_batch(&mut stacks, &crane_move),
        }
    }
    println!("The end position of the stacks is:");
    print_stacks(&stacks);
    print_top_of_stacks(&stacks);

    Ok(())
}

enum Part {
    A,
    B,
}

fn pick_part_to_solve() -> Result<Part, Box<dyn Error>> {
    let part = args().nth(1).ok_or("Do you want to solve part A or B?")?;
    match part.to_lowercase().as_str() {
        "a" => Ok(Part::A),
        "b" => Ok(Part::B),
        _ => Err("Please select a part")?,
    }
}

struct CraneMove {
    count: usize,
    from: usize,
    to: usize,
}

// print the top element of each stack
fn print_top_of_stacks(stacks: &Vec<Vec<char>>) {
    println!("Top of stacks:");
    for (_, stack) in stacks.iter().enumerate() {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}


fn print_stacks(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        for item in stack {
            print!("{}", item);
        }
        println!();
    }
    println!();
}

fn perform_crane_move(stacks: &mut Vec<Vec<char>>, crane_move: &CraneMove) {
    for _ in 0..crane_move.count {
        let temp = stacks[crane_move.from].pop().unwrap();
        stacks[crane_move.to].push(temp);
    }
}

fn perform_crane_move_batch(stacks: &mut Vec<Vec<char>>, crane_move: &CraneMove) {
    let cutoff = stacks[crane_move.from].len() - crane_move.count;
    let mut accum = stacks[crane_move.from].split_off(cutoff);
    stacks[crane_move.to].append(&mut accum);
}

// create a new crane move object from a string
impl CraneMove {
    fn new(line: &str) -> CraneMove {
        let mut parts = line.split_whitespace();
        let count = parts.next().unwrap().parse::<usize>().unwrap();
        let from = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        CraneMove { count, from, to }
    }
}

// change stacks string to vector of vectors
fn create_stacks(lines: String) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in lines.lines() {
        let chars = line.chars().collect();
        stacks.push(chars);
    }
    stacks
}

fn get_crane_moves(lines: String) -> Vec<CraneMove> {
    let mut crane_moves = Vec::new();
    for line in lines.lines() {
        let new_move = CraneMove::new(line);
        crane_moves.push(new_move);
    }
    crane_moves
}

pub fn read_input_file(filepath: String) -> Result<String, Box<dyn Error>> {
    let file_content = read_to_string(filepath)?;
    Ok(file_content)
}
