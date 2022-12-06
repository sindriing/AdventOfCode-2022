use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // let filename = "test_input.txt";
    let filename = "input.txt";
    let moves = read_moves_1(&filename).expect("Error reading moves 1");
    println!("Lets play some Rock-Paper-Scissors!");
    println!("This is the first game:");
    let mut score = 0;
    for [opp, player] in moves.iter() {
        score += sign_score(&player);
        let result = game_result(&player, &opp);
        score += result_score(&result);
    }
    println!("Score: {}", score);

    println!("This is the second game:");
    let moves = read_moves_2(&filename).expect("Error reading moves 2");
    let mut score = 0;
    let mut temp_score = 0;
    for [opp, player] in moves.iter() {
        temp_score += sign_score(&player);
        let result = game_result(&player, &opp);
        temp_score += result_score(&result);
        // println!("{:?}, {:?}, score: {:?}", &opp, &player, &temp_score);
        score += temp_score;
        temp_score = 0;
    }
    println!("Score: {}", score);
}

// Read each players move from file and outut as vector of chars
fn read_moves_1(filename: &str) -> Result<Vec<[Sign; 2]>, io::Error> {
    let mut moves = Vec::new();
    let mut lines = read_lines(filename)?;
    while let Some(line) = lines.next() {
        let line = line?;
        let c1 = line.chars().nth(0).unwrap();
        let c2 = line.chars().nth(2).unwrap();
        let m1 = opponent_char_to_sign(&c1).unwrap();
        let m2 = player_char_to_sign(&c2).unwrap();
        moves.push([m1, m2]);
    }
    Ok(moves)
}

fn read_moves_2(filename: &str) -> Result<Vec<[Sign; 2]>, io::Error> {
    let mut moves = Vec::new();
    let mut lines = read_lines(filename)?;
    while let Some(line) = lines.next() {
        let line = line?;
        let c1 = line.chars().nth(0).unwrap();
        let c2 = line.chars().nth(2).unwrap();
        let m1 = opponent_char_to_sign(&c1).unwrap();
        let r2 = player_char_to_result(&c2).unwrap();
        let m2 = move_and_result_to_player_move(&m1, r2);
        moves.push([m1, m2]);
    }
    Ok(moves)
}

fn opponent_char_to_sign(c: &char) -> Option<Sign> {
    match c {
        'A' => Some(Sign::Rock),
        'B' => Some(Sign::Paper),
        'C' => Some(Sign::Scissor),
        _ => None,
    }
}

fn player_char_to_sign(c: &char) -> Option<Sign> {
    match c {
        'X' => Some(Sign::Rock),
        'Y' => Some(Sign::Paper),
        'Z' => Some(Sign::Scissor),
        _ => None,
    }
}

fn player_char_to_result(c: &char) -> Result<GameResult, &'static str> {
    match c {
        'X' => Ok(GameResult::Lose),
        'Y' => Ok(GameResult::Draw),
        'Z' => Ok(GameResult::Win),
        err => panic!("Invalid sign for player {:?}", err),
    }
}

fn move_and_result_to_player_move(opp: &Sign, res: GameResult) -> Sign {
    match (opp, res) {
        (Sign::Rock, GameResult::Draw) => Sign::Rock,
        (Sign::Rock, GameResult::Win) => Sign::Paper,
        (Sign::Rock, GameResult::Lose) => Sign::Scissor,
        (Sign::Paper, GameResult::Draw) => Sign::Paper,
        (Sign::Paper, GameResult::Win) => Sign::Scissor,
        (Sign::Paper, GameResult::Lose) => Sign::Rock,
        (Sign::Scissor, GameResult::Draw) => Sign::Scissor,
        (Sign::Scissor, GameResult::Win) => Sign::Rock,
        (Sign::Scissor, GameResult::Lose) => Sign::Paper,
    }
}

#[derive(Debug)]
enum Sign {
    Rock,
    Paper,
    Scissor,
}

// take in RSP and return corresponding score
fn sign_score(rps: &Sign) -> i32 {
    match rps {
        Sign::Rock => 1,
        Sign::Paper => 2,
        Sign::Scissor => 3,
    }
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

fn result_score(res: &GameResult) -> i32 {
    match res {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Lose => 0,
    }
}

// calculate score of a game
fn game_result(a: &Sign, b: &Sign) -> GameResult {
    match (a, b) {
        (Sign::Rock, Sign::Rock) => GameResult::Draw,
        (Sign::Rock, Sign::Paper) => GameResult::Lose,
        (Sign::Rock, Sign::Scissor) => GameResult::Win,
        (Sign::Paper, Sign::Rock) => GameResult::Win,
        (Sign::Paper, Sign::Paper) => GameResult::Draw,
        (Sign::Paper, Sign::Scissor) => GameResult::Lose,
        (Sign::Scissor, Sign::Rock) => GameResult::Lose,
        (Sign::Scissor, Sign::Paper) => GameResult::Win,
        (Sign::Scissor, Sign::Scissor) => GameResult::Draw,
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
