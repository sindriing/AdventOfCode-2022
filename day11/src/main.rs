use rust_math::num::lcm;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use utils::{pick_part_to_solve, Part};

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
    items: VecDeque<i64>,
    test_mod: i64,
    throw_to: (usize, usize),
    inspect: fn(i64) -> i64,
    bored: bool,
    inspections: i64,
}

impl Monkey {
    fn new(
        items: Vec<i64>,
        test_mod: i64,
        throw_to: (usize, usize),
        inspect: fn(i64) -> i64,
    ) -> Monkey {
        Monkey {
            items: VecDeque::from(items),
            test_mod,
            throw_to,
            inspect,
            bored: true,
            inspections: 0,
        }
    }

    fn not_bored(&mut self) {
        self.bored = false
    }

    fn bored(item: i64) -> i64 {
        item / 3
    }

    // Takes care of inspecting as well
    fn throw(&mut self) -> Option<(i64, usize)> {
        let mut item = self.items.pop_front()?;
        self.inspections += 1;
        item = (self.inspect)(item);
        if self.bored {
            item = Monkey::bored(item);
        }
        let target = match (item % self.test_mod) == 0 {
            true => self.throw_to.0,
            false => self.throw_to.1,
        };
        Some((item, target))
    }

    fn receive(&mut self, item: i64) {
        self.items.push_back(item);
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

#[allow(dead_code)]
fn test_monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(vec![79, 98], 23, (2, 3), |x| x * 19),
        Monkey::new(vec![54, 65, 75, 74], 19, (2, 0), |x| x + 6),
        Monkey::new(vec![79, 60, 97], 13, (1, 3), |x| x * x),
        Monkey::new(vec![74], 17, (0, 1), |x| x + 3),
    ]
}

// Who needs input parsing when you can use VIM macros?
fn input_monkeys() -> Vec<Monkey> {
    vec![
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
    // let mut monkeys = test_monkeys();
    let mut monkeys = input_monkeys();
    println!("Part A:");

    let mut monkey_lcm = 1;
    for monkey in &monkeys {
        monkey_lcm = lcm(monkey_lcm, monkey.test_mod as i32);
    }

    for _ in 0..20 {
        for from in 0..monkeys.len() {
            do_round(&mut monkeys, from, monkey_lcm as i64);
        }
    }
    let monkeybusiness = find_monkeybusiness(&monkeys);

    println!("Monkey business: {}", monkeybusiness);
}

fn part_b() {
    println!("Part B");
    let mut monkeys = input_monkeys();

    // setup for part b
    let mut monkey_lcm = 1;
    for monkey in &mut monkeys {
        monkey.not_bored();
        monkey_lcm = lcm(monkey_lcm, monkey.test_mod as i32);
    }

    for _ in 0..10000 {
        for from in 0..monkeys.len() {
            do_round(&mut monkeys, from, monkey_lcm as i64);
        }
    }

    let monkeybusiness = find_monkeybusiness(&monkeys);
    println!("Monkey business: {}", monkeybusiness);
}

fn find_monkeybusiness(monkeys: &Vec<Monkey>) -> u64 {
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<i64>>();

    inspections.sort();
    inspections[inspections.len() - 2] as u64 * inspections[inspections.len() - 1] as u64
}

fn do_round(monkeys: &mut Vec<Monkey>, from: usize, monkey_lcm: i64) {
    while let Some((item, target)) = monkeys[from].throw() {
        let simplified = item % monkey_lcm;
        monkeys[target].receive(simplified);
    }
}
