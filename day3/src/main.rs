use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt").expect("Could not read file");
    let sum = sum_duplicated_items(lines);
    println!("Sum: {}", sum);
}

fn part_a() {
    let lines = read_lines("input.txt").expect("Could not read file");
    let sum = sum_duplicated_items(lines);
    println!("Sum: {}", sum);

}

fn sum_duplicated_items(mut lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut sum = 0;
    while let Some(line) = lines.next() {
        if let Ok(line) = line {
            // convert string to vector of chars
            let chars: Vec<char> = line.chars().collect();

            // split into two halves
            let half1 = chars[..chars.len()/2].to_vec();
            let mut half1 = sort_and_remove_duplicates(half1);
            let half2 = chars[chars.len()/2..].to_vec();
            let mut half2 = sort_and_remove_duplicates(half2);

            // join the halves with no duplicates
            half1.append(&mut half2);
            half1.sort();
            let dup = find_duplicate(half1);

            let prio = char_to_priority(dup.unwrap());
            match prio {
                Some(prio) => sum += prio,
                None => println!("No priority found for: {:?}", dup),
            }
        }
    }
    sum
}

// Split a vector in half and return each halves

// return a copy of the array without duplicates
fn sort_and_remove_duplicates(mut arr: Vec<char>) -> Vec<char> {
    let mut other = Vec::new();
    arr.sort();
    let mut last_char = ' ';
    
    for c in arr {
        if c == last_char {
            continue;
        } else {
            last_char = c;
            other.push(c);
        }
    }
    other
}

// Find duplicate character in sorted vector
fn find_duplicate(vec: Vec<char>) -> Option<char> {
    let mut last_char = ' ';
    for c in vec.iter() {
        if *c == last_char {
            return Some(*c);
        }
        last_char = *c;
    }
    return None;
}


// Convert a character to to it's ascii value
fn char_to_priority(c: char) -> Option<i32> {
    let c = c as u8;
    match c {
        65..=90 => Some((c - 64 + 26) as i32),
        97..=122 => Some((c - 96) as i32),
        _ => None,
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

