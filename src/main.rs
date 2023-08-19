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
