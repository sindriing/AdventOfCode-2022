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

    Ok(())
}

struct Tree {
    size: i32,
    seen: bool,
}

fn walk_grid(grid: &mut Vec<Vec<Tree>>) {
    let mut tallest;
    for row in grid.iter_mut() {
        tallest = -1;
        for tree in row.iter_mut() {
            if tree.size > tallest {
                tallest = tree.size;
                tree.seen = true;
            }
        }
        println!("");
    }
}

fn count_seen(grid: &Vec<Vec<Tree>>) -> i32 {
    let mut count = 0;
    for row in grid.iter() {
        for tree in row.iter() {
            if tree.seen {
                count += 1;
            }
        }
    }
    return count;
}

fn print_grid(grid: &Vec<Vec<Tree>>) {
    for row in grid {
        for col in row {
            print!("{}", col.size);
        }
        println!("");
    }
}

fn print_seen(grid: &Vec<Vec<Tree>>) {
    for row in grid {
        for tree in row {
            print!(
                "{}",
                match tree.seen {
                    true => "#",
                    false => ".",
                }
            );
        }
        println!("");
    }
}

fn construct_grid(lines: &mut Lines) -> Vec<Vec<Tree>> {
    let mut grid = Vec::new();
    for line in lines {
        let temp = line
            .chars()
            .map(|s| Tree {
                size: s.to_digit(10).unwrap() as i32,
                seen: false,
            })
            .collect();
        grid.push(temp);
    }
    grid
}

// transposing and reversing the grid is not efficient but it's elegant in a way
fn part_a(mut lines: &mut Lines) {
    println!("Part A:");
    let mut grid = construct_grid(&mut lines);
    walk_grid(&mut grid);

    grid = mirror(grid);
    walk_grid(&mut grid);

    grid = transpose(grid);
    walk_grid(&mut grid);

    grid = mirror(grid);
    walk_grid(&mut grid);
    print_grid(&grid);
    print_seen(&grid);

    let seen = count_seen(&grid);
    println!("Seen: {}", seen);
}

fn mirror<T>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    for row in v.iter_mut() {
        row.reverse();
    }
    v
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn part_b(_: &mut Lines) {
    println!("Part B");
}
