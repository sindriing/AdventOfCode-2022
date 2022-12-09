use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Day 4");
    part_a()
}

fn part_a() {
    let filename = "input.txt";
    let mut day1_count = 0;
    let mut day2_count = 0;
    let lines = &mut read_lines(filename).expect("sda");
    while let Some(schedules) = get_two_schedules(lines) {
        // println!("schedules: {:?}", schedules);
        if schedule_fully_overlaps(&schedules[0], &schedules[1]) {
            day1_count += 1;
        }
        if schedule_overlaps(&schedules[0], &schedules[1]) {
            day2_count += 1;
        }
    }
    println!("Part A: {}", day1_count);
    println!("Part B: {}", day2_count);
}

#[derive(Debug)]
struct Schedule {
    start: i32,
    end: i32,
}

fn schedule_fully_overlaps(s1: &Schedule, s2: &Schedule) -> bool {
    s1.start <= s2.start && s1.end >= s2.end || s2.start <= s1.start && s2.end >= s1.end
}

fn schedule_overlaps(s1: &Schedule, s2: &Schedule) -> bool {
    return schedule_fully_overlaps(s1, s2) || 
    (s2.start <= s1.start) && (s1.start <= s2.end) || 
    (s1.start <= s2.start) && (s2.start <= s1.end) ||
    (s2.start <= s1.end) && (s1.end <= s2.end) || 
    (s1.start <= s2.end) && (s2.end <= s1.end);
}

fn get_two_schedules(lines: &mut io::Lines<io::BufReader<File>>) -> Option<[Schedule; 2]> {
    let sched1;
    let sched2;

    if let Some(line) = lines.next() {
        let l = line.unwrap();
        let mut temp = l.split(',');
        sched1 = get_schedule(&temp.next().unwrap().to_string());
        sched2 = get_schedule(&temp.next().unwrap().to_string());
    } else {
        return None;
    }
    Some([sched1, sched2])
}

// parse and get one line of schedule
fn get_schedule(line: &String) -> Schedule {
    let mut iter = line.split("-");
    let sched = Schedule {start: iter.next().expect("-").parse::<i32>().expect("parse fail"),
                          end: iter.next().expect("-").parse::<i32>().expect("parse fail")};
    return sched;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

