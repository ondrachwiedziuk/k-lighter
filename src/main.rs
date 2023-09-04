//! Description of design pattern:
//! 
//! At input takes a .k file. This file is read line by line. Each line is processed to an automaton.
//! This automaton is deterministic pushdown automaton, that works recursively.
//! reads input line char by char. Each char gets its number code, which says, what type of char it is.
//! By this code is determined next styte of automaton. However, there are flags, that contains information about paranthesses nestation and about spaces.
//! If Automaton recognizes a word, gives it to an Scheme struct, which color it by given pattern and end state of automaton.
//! After end of automaton running process, writes this line as a paragraph to a HTML file.

mod symbols;
mod colors;
mod automat;
mod formatter;
use std::io::{self, BufRead};
use clap::Parser;
use std::fs;
use std::path::Path;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Using style
    #[arg(short, long, default_value_t = String::from("basic"))]
    style: String,

    /// Input file
    input: std::path::PathBuf,

    /// Output file
    output: std::path::PathBuf,
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


/// Main body.
/// 
/// takes an argument from command line and proccess a highlightation of given file.
fn main() {
    let args = Args::parse();

    if let Ok(lines) = read_lines(args.input) {
        let pattern = colors::pattern(args.style.as_str());
        let mut html = formatter::HTML::new(pattern["background"].clone());
        for line in lines {
            if let Ok(content) = line {
                let mut highlighter = automat::Highlighter::new(content, "basic");
                highlighter.start();
                let text: String = highlighter.get();
                html.p(text);
            }
        }
        fs::write(args.output, html.print()).expect("Unable to write file")

    }
}
