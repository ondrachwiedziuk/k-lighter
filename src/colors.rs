use configparser::ini::Ini;
use std::collections::HashMap;
use std::path::PathBuf;
use std::env;
use dirs;


pub fn pattern(style: &str) -> HashMap<String, String>{
    let mut config = Ini::new();
    let cargo = env::var("CARGO_HOME");
    let config_dir = match cargo {
        Ok(config_dir) => PathBuf::from(&config_dir),
        Err(_) => dirs::home_dir().unwrap().join(".cargo"),
    };
    let settings = config_dir.join("k-lighter.ini");
    let map = config.load(settings);
    match map {
        Ok(_x) => (),
        Err(_x) => return HashMap::new(),
    }
    let mut pattern = HashMap::new();
    let keys = ["verbs", "numbers", "vars", "adverbs", "reserved", "comments", "string", "pars_0", "pars_1", "pars_2", "pars_3", "background"];
    for key in keys {
        pattern.insert(key.to_string(), config.get(style, key).unwrap());
    }
    return pattern;
}


pub struct Scheme {
    pub style: String,
    pub buffer: String,
    pub pattern: HashMap<String, String>,
}

impl Scheme {
    pub fn new(style: String) -> Self {
        let s = pattern(style.as_str());
        Self { style, buffer: "".to_string(), pattern: s }
    }

    pub fn color(&mut self, word: String, key: String) {
        self.buffer.push_str("<span style=\"color:#");
        self.buffer.push_str(self.pattern[key.as_str()].as_str());
        self.buffer.push_str("\">");
        self.buffer.push_str(word.as_str());
        self.buffer.push_str("</span>");
    }

    pub fn space(&mut self) {
        self.buffer.push(' ');
    }
}
