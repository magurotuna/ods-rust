use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Parenthesis {
    Paren,
    Brace,
    Bracket,
}

impl TryFrom<char> for Parenthesis {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' | ')' => Ok(Parenthesis::Paren),
            '{' | '}' => Ok(Parenthesis::Brace),
            '[' | ']' => Ok(Parenthesis::Bracket),
            _ => Err("Cannot convert this character to a parenthesis."),
        }
    }
}

fn is_match_string(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        let p: Parenthesis = c.try_into().unwrap();
        match stack.pop() {
            Some(pop) => {
                if pop != p {
                    return false;
                }
            }
            None => stack.push(p),
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match_string() {
        assert!(is_match_string("{{()[]}}"));
        assert!(!is_match_string("{{()]}"));
    }
}
