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

fn part_b(mut lines: &mut Lines) {
    println!("Part B");
    let mut grid: Vec<Vec<TreeHouse>> = construct_grid(&mut lines);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            look_left(&mut grid[i], j);
            look_right(&mut grid[i], j);
        }
    }
    grid = transpose(grid);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            look_left(&mut grid[i], j);
            look_right(&mut grid[i], j);
        }
    }
    // Don't really need to turn it back but I find it nicer
    // to return it to the original orientation
    grid = transpose(grid);

    // print_trees(&grid);
    // println!("");
    // print_treehouse_scores(&grid);

    let best_score = find_best_scoring_treehouse(&grid);
    println!("Best score: {}", best_score);
}

fn find_best_scoring_treehouse(grid: &Vec<Vec<TreeHouse>>) -> i32 {
    let mut best_score = 0;
    for row in grid {
        for treehouse in row {
            if treehouse.score > best_score {
                best_score = treehouse.score;
            }
        }
    }
    best_score
}

fn look_left(trees: &mut Vec<TreeHouse>, origin: usize) {
    let mut i = origin as i32 -1;
    let mut score = 0;
    while i >= 0 {
        // Tree is smaller than us
        if trees[i as usize].size < trees[origin].size {
            score += 1;
        } else {
            // we've found a tree that's bigger than us
            score += 1;
            break;
        }
        i -= 1;
    }
    trees[origin].score *= score;
}

fn look_right(trees: &mut Vec<TreeHouse>, origin: usize) {
    let mut i = origin as i32 + 1;
    let mut score = 0;
    while i < trees.len() as i32 {
        if trees[i as usize].size < trees[origin].size {
            score += 1;
        } else {
            score += 1;
            break;
        }
        i += 1;
    }
    trees[origin].score *= score;
}

struct Tree {
    size: i32,
    seen: bool,
}

impl Trunk for Tree {
    fn new(s: i32) -> Tree {
        Tree { size: s, seen: false, }
    }
    fn size(&self) -> i32 {
        self.size
    }
}

trait Trunk {
    // Static method signature; `Self` refers to the implementor type
    fn new(s: i32) -> Self;
    fn size(&self) -> i32;
}

struct TreeHouse {
    size: i32,
    score: i32,
}

impl Trunk for TreeHouse {
    fn new(s: i32) -> TreeHouse {
        TreeHouse { size: s, score: 1 }
    }
    fn size(&self) -> i32 {
        self.size
    }
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

#[allow(dead_code)]
fn print_trees<T: Trunk>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for col in row {
            print!("{}", col.size());
        }
        println!("");
    }
}

#[allow(dead_code)]
fn print_treehouse_scores(grid: &Vec<Vec<TreeHouse>>) {
    for row in grid {
        for col in row {
            print!("{}", col.score);
        }
        println!("");
    }
}

#[allow(dead_code)]
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

fn construct_grid<T: Trunk>(lines: &mut Lines) -> Vec<Vec<T>> {
    let mut grid = Vec::new();
    for line in lines {
        let temp = line
            .chars()
            .map(|s| T::new(s.to_digit(10).unwrap() as i32))
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
    print_trees(&grid);
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
