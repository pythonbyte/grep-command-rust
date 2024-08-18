use std::env;
use std::io;
use std::iter::zip;
use std::process;

struct CustomIterator<'a> {
    remaining: &'a str,
}

impl<'a> CustomIterator<'a> {
    fn new(s: &'a str) -> Self {
        CustomIterator { remaining: s }
    }
}

impl<'a> Iterator for CustomIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }

        if self.remaining.starts_with("\\") {
            let (token, rest) = self.remaining.split_at(2);
            self.remaining = rest;
            Some(token)
        } else {
            let (token, rest) = self.remaining.split_at(1);
            self.remaining = rest;
            Some(token)
        }
    }
}

fn match_patt(input_line: &str, pattern: &str) -> bool {
    let mut iterator = CustomIterator::new(pattern);

    if pattern.contains("?") {
        let parts: Vec<&str> = pattern.split("?").collect();
        let first_part = parts.first().expect("Somethign wrong");
        let max_len = parts.len().saturating_sub(1);
        let pattern_remain = parts.last().expect("Somethigh");
        let actual_remain = &input_line[parts.get(0).expect("nothing").len()..];

        if input_line.contains(first_part) || input_line.contains(&first_part[..max_len]) {
            if actual_remain == *pattern_remain {
                return true;
            } else {
                return false;
            }
        }
    }

    let first_value = iterator.nth(0).unwrap_or("");

    if first_value == "[" {
        let pattern_to_match = pattern.trim_start_matches("[").trim_end_matches("]");

        if pattern_to_match.contains("^") {
            let rest = &pattern_to_match[1..];
            return !rest.chars().any(|letter| input_line.contains(letter));
        } else {
            return pattern_to_match
                .chars()
                .any(|letter| input_line.contains(letter));
        }
    } else if first_value == "^" {
        let rest = &pattern[1..];
        let input_chars: Vec<char> = input_line.chars().collect();

        for (idx, letter) in rest.char_indices() {
            if idx >= input_chars.len() || letter != input_chars[idx] {
                return false;
            }
        }
        return true;
    }

    for (idx, letter) in input_line.char_indices() {
        if match_word_patt(first_value, letter) {
            let new_iterator: Vec<_> = CustomIterator::new(pattern).collect();
            let mut is_match: bool = true;
            let remain_count = input_line.chars().skip(idx).count();

            if remain_count < new_iterator.len()
                && *new_iterator.last().unwrap() != "$"
                && !pattern.contains("+")
                && !pattern.contains("?")
                && !pattern.contains("|")
            {
                return false;
            }

            for (symbol, (index, letter)) in zip(new_iterator, input_line.char_indices().skip(idx))
            {
                if symbol == "+" {
                    break;
                }
                if symbol == "(" && pattern.contains("|") {
                    let values: &str = pattern.split("(").last().unwrap();

                    let new_values: Vec<&str> = values.trim_end_matches(")").split("|").collect();

                    let first: &str = new_values.first().expect("haha");
                    let last: &str = new_values.last().expect("uhasa");

                    let test: &str = &input_line[index..];

                    if test.contains(first) || test.contains(last) {
                        return true;
                    } else {
                        return false;
                    }
                }

                if match_word_patt(symbol, letter) {
                    continue;
                } else {
                    is_match = false;
                    break;
                }
            }
            if is_match {
                return true;
            }
        }
    }

    false
}

fn match_word_patt(pattern: &str, letter: char) -> bool {
    if pattern == "." {
        true
    } else if pattern.starts_with(r"\d") {
        return letter.is_numeric();
    } else if pattern.starts_with(r"\w") {
        return letter.is_alphabetic();
    } else if pattern == letter.to_string().as_str() {
        return true;
    } else if pattern == "(" {
        return true;
    } else {
        false
    }
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    println!("Input line: {}", input_line);
    println!("Pattern: {}", pattern);
    if match_patt(&input_line, &pattern) {
        println!("Exit 0");
        process::exit(0)
    } else {
        println!("Exit 1");
        process::exit(1)
    }
}
