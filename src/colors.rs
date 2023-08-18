use configparser::ini::Ini;
use std::collections::HashMap;


pub fn pattern(style: &str) -> HashMap<String, String>{
    let mut config = Ini::new();
    let map = config.load("./src/colors.ini");
    match map {
        Ok(_x) => (),
        Err(_x) => return HashMap::new(),
    }
    let mut pattern = HashMap::new();
    let keys = ["verbs", "numbers", "vars", "adverbs", "reserved", "comments", "string", "pars_1", "pars_2", "pars_3", "pars_4"];
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

    pub fn end_line(&mut self) {
        self.buffer.push('\n');
    }

    pub fn space(&mut self) {
        self.buffer.push(' ');
    }
}
