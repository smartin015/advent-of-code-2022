use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn fully_contains(p1: &[i32], p2: &[i32]) -> bool {
    return p1[0] <= p2[0] && p1[1] >= p2[1];
}

fn no_overlap(p1: &[i32], p2: &[i32]) -> bool {
    return 
        p2[0] > p1[1] 
        || 
        p1[0] > p2[1];
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
                let sp = l.split(',').collect::<Vec<&str>>();
                let p1: Vec<i32> = sp[0].split('-').map(|s| s.parse().expect("parse error")).collect();
                let p2: Vec<i32> = sp[1].split('-').map(|s| s.parse().expect("parse error")).collect();
                // let result: bool = fully_contains(p1.as_slice(), p2.as_slice()) || fully_contains(p2.as_slice(), p1.as_slice());
                let result: bool = !no_overlap(p1.as_slice(), p2.as_slice());
                println!("{}", result);
                if result {
                    total += 1;
                }
            }
        }
    }
    println!("Total: {}", total);
}
