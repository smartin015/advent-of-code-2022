use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn consecutive_uniques(s: String, n: usize) -> usize {
    for i in n..s.len() {
        let hs: HashSet<char> = s[i-n..i].chars().collect();
        // println!("idx {} len {}", i, hs.len());
        if hs.len() == n {
            return i; 
        }
    }
    return usize::MAX;
}

fn start_of_packet_idx(s: String) -> usize {
    return consecutive_uniques(s, 4);
}

fn start_of_message_idx(s: String) -> usize {
    return consecutive_uniques(s, 14);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let result: usize = start_of_message_idx(l);
                println!("{}", result);
            }
        }
    }
}
