mod symbols;
use std::io::{self, BufRead};


fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        let input = line.unwrap();
        for c in input.chars() {
            print!("{}", symbols::alphabet(c));

        }
        println!();
    }
}
