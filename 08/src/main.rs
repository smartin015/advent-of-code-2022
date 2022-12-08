extern crate nalgebra as na;

use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use na::DMatrix;

fn scenic_score(rr: usize, cc: usize, w: usize, h: usize, m_in: &DMatrix<u8>) -> u32 {
    let height: u8 = m_in[(rr, cc)];
    let mut score: u32 = 1;
    // Left
    let mut acc = 0;
    for c in (0..cc).rev() {
        acc += 1;
        //println!("Check left {},{}; acc now {}", rr, c, acc);
        if m_in[(rr, c)] >= height {break;}
    }
    score *= acc;

    // Right
    acc = 0;
    for c in cc+1..w {
        acc += 1;
        //println!("Check right {},{}; acc now {}", rr, c, acc);
        if m_in[(rr, c)] >= height {break;}
    }
    score *= acc;

    // Up
    acc = 0;
    for r in (0..rr).rev() {
        acc += 1;
        //println!("Check up {},{}; acc now {}", r, cc, acc);
        if m_in[(r, cc)] >= height {break;}
    }
    score *= acc;

    // Down 
    acc = 0;
    for r in rr+1..h {
        acc += 1;
        //println!("Check down {},{}; acc now {}", r, cc, acc);
        if m_in[(r, cc)] >= height {break;}
    }
    score *= acc;
    return score;
}

fn raycast(start_height: u8, rfrom: usize, rto: usize, cfrom: usize, cto: usize, m_in: &DMatrix<u8>, m_out: &mut DMatrix<bool>) {
    let mut prev_heights: u8 = start_height;
    // println!("raycast {}..{}, {}..{}:", rfrom, rto, cfrom, cto);
    let mut ir: Vec<usize> = (rfrom..(rto+1)).collect();
    if rto < rfrom {
        ir = (rto..rfrom+1).rev().collect();
    }
    let mut ic: Vec<usize> = (cfrom..(cto+1)).collect();
    if cto < cfrom {
        ic = (cto..cfrom+1).rev().collect();
    }

    for r in &ir {
        for c in &ic {
            let height = m_in[(*r, *c)];
            m_out[(*r, *c)] |= prev_heights < height;
            // println!("({}, {}) -> {} vs prev {} -> {}", r, c, height, prev_heights, m_out[(*r, *c)]);
            prev_heights = cmp::max(prev_heights, height);
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

fn main() {
    let mut w: usize = 0;
    let mut h: usize = 0;
    let mut data: Vec<u8> = Vec::new();
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                // println!("{}", l);
                let mut d: Vec<u8> = l.chars().map(|c| u8::try_from(c.to_digit(10).expect("parse error")).expect("cast error")).collect();
                w = d.len();
                h += 1;
                data.append(&mut d);
            }
        }
    }
    let trees = DMatrix::from_vec(w, h, data).transpose();

    /*
    let mut vis = DMatrix::from_element(w, h, false);
    // Edges always visible
    for r in 0..h {
        vis[(r, 0)] = true;
        vis[(r, w-1)] = true;
    }
    for c in 0..w {
        vis[(0, c)] = true;
        vis[(h-1, c)] = true;

    }

    for r in 1..h-2 {
        for c in 1..w-2 {
            raycast(trees[(r, 0)], r, r, 1, w-1, &trees, &mut vis); // Row R, left-to-right
            raycast(trees[(r, w-1)], r, r, w-1, 1, &trees, &mut vis); // Row R, right-to-left
            raycast(trees[(0, c)], 1, h-1, c, c, &trees, &mut vis); // Col C, top-to-bottom
            raycast(trees[(h-1, c)], h-1, 1, c, c, &trees, &mut vis); // Col C, bottom-to-top
        }
    }
    // println!("{}\n{}", trees, vis);
    let mut total = 0;
    for v in vis.iter() {
        if *v {
            total += 1;
        }
    }
    println!("Total: {}", total);
    */

    // let mut score = DMatrix::from_element(w, h, u32::MIN);
    let mut score: u32 = 0;
    // println!("Scenic score: {}", scenic_score(3, 2, w, h, &trees));
    for r in 0..h {
        for c in 0..w {
            score = cmp::max(score, scenic_score(r, c, w, h, &trees));
        }
    }
    // println!("{}\n{}", trees, score);
    println!("max score: {}", score);
}
