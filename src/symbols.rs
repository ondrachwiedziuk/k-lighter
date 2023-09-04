//! Alphabet of K language.
//! 
//! This module contains a function that for every char determines its number code.


/// Alphabet function.
/// 
/// Takes a character and return its number code.
/// 0:      Space
/// 1:      Verb
/// 2:      Verb with possible reserved word
/// 3:      Dot
/// 4:      Adverb
/// 5:      Doubledot
/// 6:      Letf par
/// 7:      Right par
/// 8:      Number
/// 9:      Alphabet
/// 10:     String
/// 11:     Comment
/// 100:    Unrecognized string
/// 101:    End of line
/// 
/// Args:
/// 
///     symbol: char    Character
/// 
/// Returns:
/// 
///     usize:  Number code of given character
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
