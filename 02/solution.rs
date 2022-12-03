use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn type_score(xyz: char) -> i32 {
    match xyz{
        'X' => return 1,
        'Y' => return 2,
        'Z' => return 3,
        _ => return 0,
    }
}

fn outcome_score(p1: char, p2: char) -> i32 {
    match  format!("{}{}",p1,p2).as_ref() {
        "AY"|"BZ"|"CX" => return 6, // win
        "AX"|"BY"|"CZ" => return 3, // draw
        _ => return 0, // all other conditions lose
    }
}

fn score_round(p1: char, p2: char) -> i32 {
    return type_score(p2) + outcome_score(p1, p2);
}

fn choose_move(p1: char, outcome: char) -> char {
    match format!("{}{}", p1, outcome).as_ref() {
        // Lose cases
        "AX" => return 'Z', // Lose against rock -> scissors
        "BX" => return 'X',
        "CX" => return 'Y',

        // Draw cases - play same
        "AY" => return 'X',
        "BY" => return 'Y',
        "CY" => return 'Z',

        // Win cases 
        "AZ" => return 'Y',
        "BZ" => return 'Z',
        "CZ" => return 'X',
        &_ => todo!(),
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut total = 0;
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let p1 = l.chars().nth(0).unwrap();
                let action = l.chars().nth(2).unwrap();
                total += score_round(p1, choose_move(p1, action));
                println!("{}", l);
            }
        }
    }
    println!("Total: {}", total);
}
