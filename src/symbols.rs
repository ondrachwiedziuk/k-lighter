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

pub fn reserved(word: &str) -> bool {
    return match word {
        "_T"|"_a"|"_c"|"_d"|"_f"|"_h"|"_i"|"_k"|"_n"|"_p"|"_s"|"_t"|"_v"|"_w"=> true,
        "_acos"|"_asin"|"_atan"|"_ceil"|"_ceiling"|"_cos"|"_cosh"|"_exp"|"_floor"|"_log"|"_sin"|"_sinh"|"_sqr"|"_sqrt"|"_tan"|"_tanh"|"_abs"=> true,
        "_bd"|"_ci"|"_db"|"_dj"|"_exit"|"_getenv"|"_gtime"|"_host"|"_ic"|"_inv"|"_jd"|"_lt"|"_ltime"|"_size"=> true,
        "_bin"|"_binl"|"_di"|"_dot"|"_draw"|"_dv"|"_dvl"|"_hash"|"_hat"|"_in"|"_lin"|"_lsq"|"_mul"|"_setenv"|"_sm"|"_ss"|"_sv"|"_vs"|"_vsx"|"_ssr"=> true,
        _ => false,
    }
}
