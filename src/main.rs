use std::env;
use std::io::{self};
use std::process;

pub struct Regex<'a> {
    pattern: &'a str,
}

impl<'a> Regex<'a> {
    pub fn new(pattern: &'a str) -> Self {
        Regex { pattern }
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.match_regex(self.pattern, text)
    }

    fn match_regex(&self, regexp: &str, text: &str) -> bool {
        if regexp.starts_with('^') {
            self.match_here(&regexp[1..], text)
        } else {
            text.char_indices()
                .any(|(i, _)| self.match_here(regexp, &text[i..]))
        }
    }

    fn match_here(&self, regexp: &str, text: &str) -> bool {
        if regexp.is_empty() {
            return true;
        }
        if regexp.len() > 1 && regexp.chars().nth(1) == Some('*') {
            self.match_star(regexp.chars().next().unwrap(), &regexp[2..], text)
        } else if regexp.starts_with('$') && regexp.len() == 1 {
            text.is_empty()
        } else if !text.is_empty()
            && (regexp.starts_with('.') || regexp.starts_with(text.chars().next().unwrap()))
        {
            self.match_here(&regexp[1..], &text[1..])
        } else {
            false
        }
    }

    fn match_star(&self, c: char, regexp: &str, text: &str) -> bool {
        text.char_indices()
            .any(|(i, ch)| (c == '.' || c == ch) && self.match_here(regexp, &text[i + 1..]))
            || self.match_here(regexp, text)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "-E" {
        eprintln!("Usage: {} -E <pattern>", args[0]);
        process::exit(1);
    }
    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    let regex = Regex::new(pattern.as_str());
    println!("Pattern: {}", regex.pattern);
    println!("Text: {}", input_line);
    println!("Match: {}", regex.is_match(input_line.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_match() {
        let regex = Regex::new("abc");
        assert!(regex.is_match("abc"));
        assert!(regex.is_match("xabcy"));
        assert!(!regex.is_match("ac"));
    }

    #[test]
    fn test_start_anchor() {
        let regex = Regex::new("^abc");
        assert!(regex.is_match("abcdef"));
        assert!(!regex.is_match("xabcdef"));
    }

    #[test]
    fn test_end_anchor() {
        let regex = Regex::new("abc$");
        assert!(regex.is_match("xabc"));
        assert!(!regex.is_match("abcx"));
    }

    #[test]
    fn test_star() {
        let regex = Regex::new("a*b");
        assert!(regex.is_match("b"));
        assert!(regex.is_match("ab"));
        assert!(regex.is_match("aaab"));
        assert!(!regex.is_match("a"));
    }

    #[test]
    fn test_dot() {
        let regex = Regex::new("a.c");
        assert!(regex.is_match("abc"));
        assert!(regex.is_match("axc"));
        assert!(!regex.is_match("ac"));
    }
}
