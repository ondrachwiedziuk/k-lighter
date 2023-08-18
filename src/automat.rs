use crate::colors::Scheme;
use crate::symbols;

pub struct Automat {
    input: String,
    buffer: String,
    index: usize,
    par: usize,
    scheme: Scheme,
    free: bool,
}

impl Automat {
    pub fn new(input: String, style: &str) -> Self {
        let scheme = Scheme::new(style.to_string());
        Self { input, buffer: "".to_string(), index: 0, par: 0 , scheme: scheme, free: true}
    }

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

    pub fn get(& mut self) -> String {
        let text = self.scheme.buffer.clone();
        self.scheme.buffer = "".to_string();
        return text;
    }

    pub fn add(&mut self, input: String) {
        self.input.push_str(input.as_str());
    }

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
                let text = format!("par_{}", self.par % 4);
                self.scheme.color(self.buffer.clone(), text.to_string());
                self.par += 1;
                self.start();
            }
            7 => {
                self.free = false;
                let text = format!("par_{}", self.par % 4);
                self.scheme.color(self.buffer.clone(), text.to_string());
                self.par -= 1;
                self.start();
            }
            8 => {
                self.free = false;
                self.n();
            }
            9 => {
                self.free = false;
                self.p();
            }
            10 => {
                self.free  = false;
                self.s();
            }
            11 => {
                if self.free {
                    self.c();
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
                self.start();
            }
        }
    }

    fn p(&mut self) {
        let literal = self.transition();
        match literal {
            8 | 9 => self.p(),

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

    fn under(&mut self) {
        let literal = self.transition();
        match literal {
            9 => self.r(),

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

    fn r(&mut self) {
        let literal = self.transition();
        match literal {
            9 => self.r(),

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

    fn n(&mut self) {
        let literal = self.transition();
        match literal {
            3 => self.number(),
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

    fn number(&mut self) {
        let literal = self.transition();
        match literal {
            3 => self.number(),
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

    fn s(&mut self) {
        let literal = self.transition();
        match literal {
            10 | 101 => {
                self.scheme.color(self.buffer.clone(), "string".to_string());
                self.buffer = "".to_string();
            }

            _ => self.s(),
        }
    }

    fn c(&mut self) {
        let literal = self.transition();
        match literal {
            101 => {
                self.scheme.color(self.buffer.clone(), "comments".to_string());
                self.buffer = "".to_string();
            }

            _ => self.c(),
        }
    }
}