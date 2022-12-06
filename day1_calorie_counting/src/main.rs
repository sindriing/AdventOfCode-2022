use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Lets go count some elf calories!");
    // let answer = get_top_calories("test_input.txt");
    let answer = get_top_calories("input.txt");
    println!("The fattest elf has {} calories", answer[0]);
    println!("The three fattest have {} calories combined", answer.iter().sum::<i32>() );
}

fn get_top_calories(filename: &str) -> [i32; 3] {
    // File hosts must exist in current path before this produces output
    let mut top_three = [0, 0, 0];
    let mut elf_cals: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    update_top_three(& mut top_three, elf_cals);
                    elf_cals = 0;
                }
                else {
                    elf_cals += ip.parse::<i32>().unwrap();
                }
            }
        }
        if elf_cals != 0 {
            update_top_three(& mut top_three, elf_cals);
        }
    }
    top_three.sort_by(|a,b| b.cmp(a));
    return top_three;
}

fn update_top_three(top_three: &mut[i32], new_val: i32) {
    for i in 0..top_three.len() {
        if top_three[i] < new_val {
            top_three[i] = new_val;
            top_three.sort();
            break;
        }
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
