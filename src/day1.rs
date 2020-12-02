use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

/**
* --- Day 1: Report Repair ---
* -------- TASK 1 ----------
* After saving Christmas five years in a row, you've decided to take
* a vacation at a nice resort on a tropical island.
* Surely, Christmas will go on without you.
*
* The tropical island has its own currency and is entirely cash-only.
* The gold coins used there have a little picture of a starfish;
* the locals just call them stars.
* None of the currency exchanges seem to have heard of them, but somehow,
* you'll need to find fifty of these coins by the time you arrive so you
* can pay the deposit on your room.
*
* Before you leave, the Elves in accounting just need you to fix your expense report.
* (your puzzle input); apparently, something isn't quite adding up.

* Specifically, they need you to find the two entries that sum to 2020 and then multiply
* those two numbers together.
*
* -------- TASK 2 ----------
* The Elves in accounting are thankful for your help;
* one of them even offers you a starfish coin they had left over from a past vacation.
* They offer you a second one if you can find three numbers in your expense report that
* meet the same criteria.
*/

const TARGET: i32 = 2020; // From description.
fn main() {
    let mut previous = HashSet::new();

    // TASK 1
    let inputfile = std::env::args().nth(1).expect("Requires a file as input");
    let file = File::open(inputfile).expect("Couldn't open file");
    let iter_lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap());

    for line in iter_lines {
        let other = TARGET - line;
        if !previous.contains(&other) {
            previous.insert(line);
        } else {
            println!("{}", line * other);
            break;
        }
    }

    // TASK 2
    let mut previous = HashMap::new();

    // TASK 1
    let inputfile = std::env::args().nth(1).expect("Requires a file as input");
    let file = File::open(inputfile).expect("Couldn't open file");
    let iter_lines: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    for i in 0..iter_lines.len() {
        for j in (i + 1)..iter_lines.len() {
            previous.insert(iter_lines[i] + iter_lines[j], (i, j));
        }
    }
    for i in 0..iter_lines.len() {
        let other = TARGET - iter_lines[i];
        if previous.contains_key(&other) {
            let tup = previous[&other];
            if tup.0 == i || tup.1 == i {
                continue;
            }
            println!("{}", iter_lines[i] * iter_lines[tup.0] * iter_lines[tup.1]);
            break;
        }
    }
}
