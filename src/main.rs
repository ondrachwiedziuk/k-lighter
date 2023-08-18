mod symbols;
mod colors;
mod automat;
use std::io::{self, BufRead};


fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut highlighter = automat::Automat::new("".to_string(), "basic");
    while let Some(line) = lines.next() {
        let input = line.unwrap();
        highlighter.add(input);
        highlighter.start();
        // for c in input.chars() {
        //     print!("{}", symbols::alphabet(c));
        // }
    }
    print!("{}", highlighter.get());
}
