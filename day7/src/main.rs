// use utils::{Part, pick_part_to_solve, read_input_file};
use utils::read_input_file;
use std::error::Error;
use std::str::Lines;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "input.txt".to_string();
    let input = read_input_file(filename)?;
    let mut lines = input.lines();
    let mut grand_total = 0;
    traverse(&mut lines, &mut grand_total);

    println!("grand_total: {}", grand_total);
    Ok(())
}

fn traverse(lines: &mut Lines, grand_total: &mut i32) -> i32 {
    // let line_ = lines.next();
    let mut accum = 0;
    while let Some(line) = lines.next(){
        if line == "$ cd .." {
            // println!("Going back");
            break;
        } else if line[..4] == *"$ cd" {
            // println!("Going down into directory {}", line[5..].trim());
            accum += traverse(lines, grand_total);
        } else if line[..4] == *"$ ls" {
            // println!("Listing directory {}", line[4..].trim());
        } else if line[..4] == *"dir " {
            // println!("found directory {}", line[4..].trim());
            continue;
        } else {
            let filesize = line.split_whitespace().next().unwrap().parse::<i32>().unwrap();
            accum += filesize;
        }
    }
    if accum <= 100000 {
        *grand_total += accum;
    }
    println!("accum: {}", accum);
    return accum;
}

// Start by just implementing the traversal
/*
* Recurse
* exit case: parse ".."
* File = sum("ls") + sum(children files)
*/
