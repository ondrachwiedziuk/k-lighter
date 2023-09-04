//! This module creates a scheme from a config.ini file.

use configparser::ini::Ini;
use std::collections::HashMap;
use std::path::PathBuf;
use std::env;
use dirs;


/// Pattern function creates a Style Hash Map from given name
/// 
/// Args.
/// 
///     style: &str     Name of given style.
/// 
/// Returns:
/// 
///     HashMap<String, String>:    HashMap where keys are names of given directives and values are colors by given style.
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


/// Scheme struct.
/// Contains a style taken from file and colors given words by that pattern.
/// 
/// Params:
/// 
///     style: String   Name of required style.
///     buffer: String  Contains a word which is supposed to color.
///     pattern: HashMap<String, String>    HashMap where keys are names of given directives and values are colors by given style.
/// 
pub struct Scheme {
    pub style: String,
    pub buffer: String,
    pub pattern: HashMap<String, String>,
}

/// Scheme implementation.
impl Scheme {

    /// Contructor of Scheme.
    /// 
    /// Args:
    /// 
    ///     style: String   Name of used style.
    /// 
    /// Returns:
    /// 
    ///     Self
    pub fn new(style: String) -> Self {
        let s = pattern(style.as_str());
        Self { style, buffer: "".to_string(), pattern: s }
    }


    /// Color function push a html fragment containing colored word to a buffer.
    /// 
    /// Args:
    /// 
    ///     word: String    Word supposed to color.
    ///     key: String     Says how was the word determined by automaton.
    pub fn color(&mut self, word: String, key: String) {
        self.buffer.push_str("<span style=\"color:#");
        self.buffer.push_str(self.pattern[key.as_str()].as_str());
        self.buffer.push_str("\">");
        self.buffer.push_str(word.as_str());
        self.buffer.push_str("</span>");
    }

    /// Space function.
    /// Adds space to buffer.
    pub fn space(&mut self) {
        self.buffer.push(' ');
    }
}
