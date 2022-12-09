// use std::{char, error::Error, fs::read_to_string};
use std::{env::args, error::Error, fs::read_to_string};

pub fn read_input_file(filepath: String) -> Result<String, Box<dyn Error>> {
    let file_content = read_to_string(filepath)?;
    Ok(file_content)
}

pub enum Part {
    A,
    B,
}

pub fn pick_part_to_solve() -> Result<Part, Box<dyn Error>> {
    let part = args().nth(1).ok_or("Do you want to solve part A or B?\n")?;
    match part.to_lowercase().as_str() {
        "a" => Ok(Part::A),
        "b" => Ok(Part::B),
        _ => Err("Please select a part")?,
    }
}
