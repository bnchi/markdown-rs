use alloc::{format, string::String, vec::Vec};
use regex::Regex;

pub struct SafeConfig<'a> {
    pub before: &'a str,
    pub after: &'a str,
    pub encode: Option<char>,
}

impl<'a> SafeConfig<'a> {
    pub(crate) fn new(before: &'a str, after: &'a str, encode: Option<char>) -> Self {
        SafeConfig {
            before,
            after,
            encode,
        }
    }
}

pub struct EscapeInfos {
    pub before: bool,
    pub after: bool,
}

pub fn escape_backslashes(value: &str, after: &str) -> String {
    let expression = Regex::new(r"\\[!-/:-@\[-`{-~]").unwrap();
    let mut results: String = String::new();
    let whole = format!("{}{}", value, after);

    let positions: Vec<usize> = expression.find_iter(&whole).map(|m| m.start()).collect();
    let mut start = 0;

    for position in &positions {
        if start != *position {
            results.push_str(&value[start..*position]);
        }

        results.push('\\');

        start = *position;
    }

    results.push_str(&value[start..]);

    results
}
