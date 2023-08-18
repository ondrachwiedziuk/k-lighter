pub fn alphabet(symbol: char) -> usize {
    return match symbol {
        // spaces
        ' ' | '\n' => 0,
        // verbs
        '+' | '-' | '*' | '%' | '|' | '&' | '^' | '!' | '>' | '=' | '~' | '@' | '?' | ',' | '#' | '$'  => 1,
        // verb with possible reserved word
        '_' => 2,
        // dot
        '.' => 3,
        // adverbs
        '\'' | '\\' => 4,
        // doubledot
        ':' => 5,
        // left pars
        '(' | '[' | '{' => 6,
        // right pars
        ')' | ']' | '}' => 7,
        // numbers
        '0'..='9' => 8,
        // alphabet
        'a'..='z' => 9,
        'A'..='Z' => 9,
        // string
        '"' => 10,
        // comment
        '/' => 11,
        // unrecognized symbols
        _ => 100,
    }
}
