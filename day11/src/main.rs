use utils::{pick_part_to_solve, read_input_file, Part};
use std::error::Error;
// use std::str::Lines;
use std::fmt;
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn Error>> {
    // let filename = "input.txt".to_string();
    // let input = read_input_file(filename)?;
    // let mut lines = input.lines();
    let part = pick_part_to_solve()?;
    if part == Part::A {
        part_a();
    } else if part == Part::B {
        // part_b(&mut lines);
        part_b();
    } else {
        Err("Please select a part [A/B]")?;
    }

    Ok(())
}

struct Monkey {
    items: VecDeque<i32>,
    test_mod: i32,
    throw_to: (usize, usize),
    inspect: fn(i32) -> i32,
    inspections: i32,
}

impl Monkey {
    fn new(items: Vec<i32>, test_mod: i32, throw_to: (usize, usize), inspect: fn(i32) -> i32) -> Monkey {
        Monkey {
            items: VecDeque::from(items),
            test_mod,
            throw_to,
            inspect,
            inspections: 0,
        }
    }

    fn bored(item: i32) -> i32 {
        item / 3
    }

    // Takes care of inspecting as well
    fn throw(&mut self) -> Option<(i32, usize)> {
        let mut item = self.items.pop_front()?;
        self.inspections += 1;
        item = (self.inspect)(item);
        item = Monkey::bored(item);
        let target = match (item % self.test_mod) == 0 {
            true => self.throw_to.0,
            false => self.throw_to.1,
        };
        Some((item, target))
    }

    fn receive(&mut self, item: i32) {
        self.items.push_back(item);
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

fn test_monkeys() -> Vec<Monkey> {
    vec! [
        Monkey::new(vec![79, 98], 23, (2, 3), |x| x * 19),
        Monkey::new(vec![54, 65, 75, 74], 19, (2, 0), |x| x + 6),
        Monkey::new(vec![79, 60, 97], 13, (1, 3), |x| x * x),
        Monkey::new(vec![74], 17, (0, 1), |x| x + 3),
    ]
}

fn input_monkeys() -> Vec<Monkey> {
    vec! [
        Monkey::new(vec![93, 98], 19, (5, 3), |x| x * 17),
        Monkey::new(vec![95, 72, 98, 82, 86], 13, (7, 6), |x| x + 5),
        Monkey::new(vec![85, 62, 82, 86, 70, 65, 83, 76], 5, (3, 0), |x| x + 8),
        Monkey::new(vec![86, 70, 71, 56], 7, (4, 5), |x| x + 1),
        Monkey::new(vec![77, 71, 86, 52, 81, 67], 17, (1, 6), |x| x + 4),
        Monkey::new(vec![89, 87, 60, 78, 54, 77, 98], 2, (1, 4), |x| x * 7),
        Monkey::new(vec![69, 65, 63], 3, (7, 2), |x| x + 6),
        Monkey::new(vec![89], 11, (0, 2), |x| x * x),
    ]
}

fn part_a() {
    // There are only 8 monkeys so I might as well skip creating a parser
    // let mut monkeys = test_monkeys();
    let mut monkeys = input_monkeys();
    println!("Part A:");

    for m in &monkeys {
        println!("Monkey: {}", m);
    }
    println!("");

    // do this 20 times
    for _ in 0..20 {
        for from in 0..monkeys.len() {
            do_round(&mut monkeys, from);
        }
    }

    for m in &monkeys {
        println!("Monkey: {}", m);
    }

    // show monkey inspections
    for m in monkeys {
        println!("Monkey inspected {} items", m.inspections);
    }
}

fn do_round(monkeys: &mut Vec<Monkey>, from: usize) {
    while let Some((item, target)) = monkeys[from].throw() {
        monkeys[target].receive(item);
    }
}

// fn part_b(mut lines: &mut Lines) {
fn part_b() {
    println!("Part B");
}

