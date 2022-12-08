use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    println!("Part A:");
    let lines = read_lines("input.txt").expect("Could not read file");
    let sum = sum_duplicated_items(lines);
    println!("Sum: {}", sum);
    println!("---------------------------------");
}

fn part_b() {
    println!("Part B:");
    let mut lines = read_lines("input.txt").expect("Could not read file");
    let mut sum = 0;
    while let Some(mut group) = get_three_lines(&mut lines) {
        group.sort();
        if let Some(key) = find_triple(group) {
            sum += char_to_priority(key).unwrap();
        } else {
            panic!("No key found");
        }
    }
    println!("Sum: {}", sum);
}

fn get_three_lines(lines: &mut io::Lines<io::BufReader<File>>) -> Option<Vec<char>> {
    let mut group: Vec<char> = vec![];

    for _ in 0..3 { // change it to get range
        if let Some(line) = lines.next() {
            // line = line.expect("Failed to parse line in get_three_lines");
            let chars: Vec<char> = line.expect("bla").chars().collect();
            let mut chars = remove_duplicates_from_elves(chars);
            group.append(&mut chars);
        } else {
            return None;
        }
    }
    Some(group)
}

fn remove_duplicates_from_elves(chars: Vec<char>) -> Vec<char> {
    // split into two halves
    let half1 = chars[..chars.len()/2].to_vec();
    let mut half1 = sort_and_remove_duplicates(half1);
    let half2 = chars[chars.len()/2..].to_vec();
    let mut half2 = sort_and_remove_duplicates(half2);
    // join the halves with no duplicates
    half1.append(&mut half2);
    half1.sort();
    half1
}

fn sum_duplicated_items(mut lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut sum = 0;
    while let Some(line) = lines.next() {
        if let Ok(line) = line {
            // convert string to vector of chars
            let chars: Vec<char> = line.chars().collect();
            let no_duplicates = remove_duplicates_from_elves(chars);
            let dup = find_duplicate(no_duplicates);
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

// find a character in a sorted array that appears three times
fn find_triple(arr: Vec<char>) -> Option<char> {
    let mut last_char = ' ';
    let mut count = 0;
    for c in arr {
        if c == last_char {
            count += 1;
        } else {
            last_char = c;
            count = 0;
        }
        if count == 2 {
            return Some(c);
        }
    }
    None
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

