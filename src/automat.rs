
//! This module contains an automaton such that processes a given String.
//! Automaton works recursively.

use crate::colors::Scheme;
use crate::symbols;


/// Automaton struct.
/// 
/// Params:
/// 
///     input: String   String which is processed by an automaton.
///     buffer: String  Buffer contains a word recognized by an automaton.
///     index: usize    Pointer to the current char in input.
///     par: usize      Counter of nestation of parentheses.
///     pub scheme: Scheme  Color scheme which is used.
///     free: bool      Flag if previous char was a space or new line.
pub struct Highlighter {
    input: String,
    buffer: String,
    index: usize,
    par: usize,
    pub scheme: Scheme,
    free: bool,
}

/// Implementation of methods of Highlighter
impl Highlighter {
    /// Constructor of Highlighter
    /// 
    /// Args:
    /// 
    ///     input: String   String which is processed by an automaton.
    ///     style: &str     Name of style which is supposed to use.
    /// 
    /// Returns:
    /// 
    ///     Self
    pub fn new(input: String, style: &str) -> Self {
        let scheme = Scheme::new(style.to_string());
        Self { input, buffer: "".to_string(), index: 0, par: 0 , scheme: scheme, free: true}
    }

    /// Transition function of automaton.
    /// 
    /// Moves a pointer by one and returns recognize of character.
    /// 
    /// Returns:
    /// 
    ///     usize: Number code of character, can be found in symbols.rs.
    fn transition(&mut self) -> usize {
        let literal = self.input.chars().nth(self.index);
        match literal {
            Some(symbol) => {
                self.buffer.push(symbol);
                self.index += 1;
                return symbols::alphabet(symbol);
            }
            None => {
                return 101;
            }
        }
    }

    /// Returns a recognized word from buffer and clears a buffer.
    /// 
    /// Returns:
    /// 
    ///     String: recognized word.
    pub fn get(& mut self) -> String {
        let text = self.scheme.buffer.clone();
        self.scheme.buffer = "".to_string();
        return text;
    }

    /// Initialization of string processing, init state of automaton.
    /// 
    /// Is able to recognize verbs, adverbs and parentheses.
    /// Calls next state by number code of given symbol.
    pub fn start(&mut self) {
        let literal = self.transition();
        match literal {
            0 => {
                self.free = true;
                self.scheme.space();
                self.buffer.pop();
                self.start();
            }
            1 | 3 => {
                self.free = false;
                self.scheme.color(self.buffer.clone(), "verbs".to_string());
                self.buffer.pop();
                self.start();
            }
            2 => {
                self.free = false;
                self.under();
            }
            4 | 5 => {
                self.free = false;
                self.scheme.color(self.buffer.clone(), "adverbs".to_string());
                self.buffer.pop();
                self.start();
            }
            6 => {
                self.free = false;
                let text = format!("pars_{}", self.par % 4);
                self.scheme.color(self.buffer.clone(), text.to_string());
                self.buffer.pop();
                self.par += 1;
                self.start();
            }
            7 => {
                self.free = false;
                self.par -= 1;
                let text = format!("pars_{}", self.par % 4);
                self.scheme.color(self.buffer.clone(), text.to_string());
                self.buffer.pop();
                self.start();
            }
            8 => {
                self.free = false;
                self.number();
            }
            9 => {
                self.free = false;
                self.var();
            }
            10 => {
                self.free  = false;
                self.string();
            }
            11 => {
                if self.free {
                    self.comment();
                }
                else {
                    self.scheme.color(self.buffer.clone(), "adverbs".to_string());
                    self.buffer.pop();
                    self.start();
                }
            }
            101 => self.free = true,
            _ => {
                self.free = false;
                self.buffer.pop();
                self.start();
            }
        }
    }

    /// Variable state.
    /// 
    /// Given word recognizes as variable.
    /// Calls next state by number code of given symbol.
    fn var(&mut self) {
        let literal = self.transition();
        match literal {
            8 | 9 => self.var(),

            101 => {
                self.scheme.color(self.buffer.clone(), "vars".to_string());
                self.buffer = "".to_string();
            }

            _ => {
                self.index -= 1;
                self.buffer.pop();
                self.scheme.color(self.buffer.clone(), "vars".to_string());
                self.buffer = "".to_string();
                self.start();
            }
        }
    }

    /// Underscore state.
    /// 
    /// Recognizes an underscore
    /// Calls next state by number code of given symbol.
    fn under(&mut self) {
        let literal = self.transition();
        match literal {
            9 => self.reserved(),

            101 => {
                self.scheme.color(self.buffer.clone(), "verbs".to_string());
                self.buffer = "".to_string();
            }

            _ => {
                self.index -= 1;
                self.buffer.pop();
                self.scheme.color(self.buffer.clone(), "verbs".to_string());
                self.buffer = "".to_string();
                self.start();
            }
        }
    }

    /// Reserved word state.
    /// 
    /// Recognizes reserved words.
    /// Calls next state by number code of given symbol.
    fn reserved(&mut self) {
        let literal = self.transition();
        match literal {
            9 => self.reserved(),

            101 => {
                self.scheme.color(self.buffer.clone(), "reserved".to_string());
                self.buffer = "".to_string();
            }

            _ => {
                self.index -= 1;
                self.buffer.pop();
                self.scheme.color(self.buffer.clone(), "reserved".to_string());
                self.buffer = "".to_string();
                self.start();
            }
        }
    }

    /// Number state.
    /// 
    /// Recognizes numbers.
    /// Calls next state by number code of given symbol.
    fn number(&mut self) {
        let literal = self.transition();
        match literal {
            3 => self.int(),
            5 => {
                self.scheme.color(self.buffer.clone(), "adverbs".to_string());
                self.buffer = "".to_string();
                self.start();
            }
            9 => self.float(),

            101 => {
                self.scheme.color(self.buffer.clone(), "numbers".to_string());
                self.buffer = "".to_string();
            }

            _ => {
                self.index -= 1;
                self.buffer.pop();
                self.scheme.color(self.buffer.clone(), "numbers".to_string());
                self.buffer = "".to_string();
                self.start();
            }
        }
    }

    /// Integer state.
    /// 
    /// Recognizes integers.
    /// Calls next state by number code of given symbol.
    fn int(&mut self) {
        let literal = self.transition();
        match literal {
            3 => self.int(),
            9 => self.float(),

            101 => {
                self.scheme.color(self.buffer.clone(), "numbers".to_string());
                self.buffer = "".to_string();
            }

            _ => {
                self.index -= 1;
                self.buffer.pop();
                self.scheme.color(self.buffer.clone(), "numbers".to_string());
                self.buffer = "".to_string();
                self.start();
            }
        }
    }

    /// Float state.
    /// 
    /// Recognizes floats.
    /// Calls next state by number code of given symbol.
    fn float(&mut self) {
        let literal = self.transition();
        match literal {
            3 => self.float(),

            101 => {
                self.scheme.color(self.buffer.clone(), "numbers".to_string());
                self.buffer = "".to_string();
            }

            _ => {
                self.index -= 1;
                self.buffer.pop();
                self.scheme.color(self.buffer.clone(), "numbers".to_string());
                self.buffer = "".to_string();
                self.start();
            }
        }
    }

    /// String state.
    /// 
    /// Recognizes strings.
    /// Calls next state by number code of given symbol.
    fn string(&mut self) {
        let literal = self.transition();
        match literal {
            10 | 101 => {
                self.scheme.color(self.buffer.clone(), "string".to_string());
                self.buffer = "".to_string();
            }

            _ => self.string(),
        }
    }

    /// Comment state.
    /// 
    /// Recognizes commnents.
    /// Calls next state by number code of given symbol.
    fn comment(&mut self) {
        let literal = self.transition();
        match literal {
            101 => {
                self.scheme.color(self.buffer.clone(), "comments".to_string());
                self.buffer = "".to_string();
            }

            _ => self.comment(),
        }
    }
}