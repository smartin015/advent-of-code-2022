use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Display = [String; 6];

fn cycles_needed(s: &str) -> i32 {
    if s == "addx" {
        return 2;
    } else {
        return 1;
    }
}

fn signal_strength(x: i32, c: i32) -> i32 {
    return x*c;
}

fn eval(screen: &mut Display, s: &String, x: &mut i32, c: &mut i32, ssc: i32) -> Option<i32> {
    let split: Vec<&str> = s.split_whitespace().collect();
    let mut r = 0;
    let mut rset = false;

    for _ in 0..cycles_needed(split[0]) {
        let sr: usize = (((*c-1) / 40) % 6).try_into().unwrap();
        let sc: usize = (((*c-1)%40) + 1).try_into().unwrap();
        if (*x-1..*x+2).contains(&(sc as i32)) {
            println!("c={}\t{}<{}<{}\tr/c={},{} blit!", *c, *x-1, *c, *x+1, sr, sc);
            set_pixel(screen, sr, sc);
        } else {
            println!("c={}\t{}<{}<{}\tr/c={},{}", *c, *x-1, *c, *x+1, sr, sc);
        }

        *c += 1;

        if *c == ssc {
            r = signal_strength(*x, *c);
            rset = true;
            // println!("x={}, c={}, ss={}", *x, *c, r);
        }
    }

    if split[0] == "addx" {
        *x += split[1].parse::<i32>().expect("integer parse error"); 
    } else if split[0] == "noop" {
    } else {
        panic!("Unsupported operator: {}", s);
    }
    if rset {
        return Some(r);
    } else {
        return None;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn set_pixel(screen: &mut Display, r: usize, c: usize) {
    screen[r].replace_range(c..c+1, "#");
}

fn display(screen: &Display) {
    println!("DISPLAY:");
    for row in screen {
        println!("{}", row);
    }
}

fn main() {
    if let Ok(lines) = read_lines("./data.txt") {
        let mut x: i32 = 1;
        let mut cycles: i32 = 0;
        let mut screen: Display = Default::default();
        for i in 0..screen.len() {
            screen[i] = (0..40).map(|_| ".").collect::<String>();
        }
        display(&screen);
        

        let ssc = [20, 60, 100, 140, 180, 220, -1];
        let mut si: usize = 0;
        let mut total = 0;
        for line in lines {
            if let Ok(l) = line {
                match eval(&mut screen, &l, &mut x, &mut cycles, ssc[si]) {
                    Some(ss) => {
                        println!("#### signal {} at {}", ss, ssc[si]);
                        si += 1;
                        total += ss;
                    }
                    None => {}
                }
                // println!("{} -> x={} c={}", l, x, cycles);
            }
        }
        println!("Total: {}", total);
        display(&screen);
    }
}
