pub fn alphabet(symbol: char) -> i32 {
    return match symbol {
        ' ' | '\n' => 0,
        '+' | '-' | '*' | '%' | '|' | '&' | '^' | '!' | '>' | '=' | '~' | '@' | '?' | '_' | ',' | '#' | '$' | '.' => 1,
        '/' | '\\' | '\'' | ':' => 2,
        '(' | '[' | '{' => 3,
        ')' | ']' | '}' => 4,
        '0'..='9' => 5,
        'a'..='z' => 6,
        'A'..='Z' => 6,
        _ => -1,
    }
}

pub fn reserved(word: &str) -> bool {
    return match word {
        "kuk" => true,
        _ => false,
    }
}