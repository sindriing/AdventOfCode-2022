use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn main() {
    println!("Lets go count some elf calories!");
    let answer = read_calories("input.txt");
    println!("The answer is {}", answer);
}

fn read_calories(filename: &str) -> i32 {
    // File hosts must exist in current path before this produces output
    let mut most_cals = 0;
    let mut elf_cals: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    most_cals = cmp::max(most_cals, elf_cals);
                    elf_cals = 0;
                }
                else {
                    elf_cals += ip.parse::<i32>().unwrap();
                }
            }
        }
        if elf_cals != 0 {
            most_cals = cmp::max(most_cals, elf_cals);
        }
    }
    return most_cals;
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
