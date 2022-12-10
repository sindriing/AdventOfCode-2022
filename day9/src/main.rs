pub mod node;

pub use node::{Direction, Location, Node};
use std::error::Error;
use std::str::Lines;
use utils::{pick_part_to_solve, read_input_file, Part};

// use node::Node;

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

    Ok(())
}

fn part_a(mut lines: &mut Lines) {
    println!("Part A:");
    let actions = expand_input(&mut lines);
    let mut head = Node::new(0, 0);
    let mut tail = Node::new(0, 0);

    for d in actions {
        head.move_relative(d);
        tail.follow(&head);
        // println!("Going: {:?}, head: {:}, tail: {:}", d, head, tail);
    }
    println!("Number of unique locations visited: {}", tail.visited.len());
}

fn expand_input(lines: &mut Lines) -> Vec<Direction> {
    let mut actions = Vec::new();
    for line in lines {
        let dir = match line.chars().nth(0) {
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let dist = line[2..].parse::<usize>().unwrap();
        let mut temp = vec![dir; dist];
        actions.append(&mut temp);
    }
    actions
}

fn part_b(mut lines: &mut Lines) {
    println!("Part B:");
    let actions = expand_input(&mut lines);
    let mut head = Node::new(0, 0);
    let mut tails = vec![Node::new(0, 0); 9];

    for d in actions {
        head.move_relative(d);
        tails[0].follow(&head);
        for i in 1..tails.len() {
            let loc = &Location {
                x: tails[i - 1].x,
                y: tails[i - 1].y,
            };
            tails[i].follow_loc(loc);
        }
        // println!("Going: {:?}, head: {:}, tail: {:}", d, head, tail);
    }
    println!(
        "Number of unique locations visited: {}",
        tails.last().unwrap().visited.len()
    );
}
