use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let config_dir = env::var("CARGO_HOME").unwrap();
    let settings = Path::new(&config_dir).join("k-lighter.ini");
    let mut f = File::create(&settings).unwrap();
    f.write_all(b"[basic]
numbers = ffffff
vars = ffff00
verbs = 00ffff
adverbs = ff00ff
reserved = ff0000
pars_0 = 006600
pars_1 = 009900
pars_2 = 00cc00
pars_3 = 00ff00
comments = 0000ff
string = 508050
background = 000000
").unwrap();
}
