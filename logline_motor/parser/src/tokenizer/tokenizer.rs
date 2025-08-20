use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Token {
    Ident(String),
    Symbol(char),
    String(String),
    Number(f64),
}

pub struct Tokenizer<'a> {
    chars: std::str::Chars<'a>,
    peeked: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { chars: input.chars(), peeked: None }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.peeked.take() {
            Some(c)
        } else {
            self.chars.next()
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.chars.next();
        }
        self.peeked
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek_char(), Some(c) if c.is_whitespace()) {
            self.next_char();
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let ch = self.next_char()?;
        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut ident = String::from(ch);
            while let Some(c) = self.peek_char() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    ident.push(self.next_char().unwrap());
                } else {
                    break;
                }
            }
            return Some(Token::Ident(ident));
        }
        if ch.is_ascii_digit() {
            let mut num = String::from(ch);
            while let Some(c) = self.peek_char() {
                if c.is_ascii_digit() || c == '.' {
                    num.push(self.next_char().unwrap());
                } else {
                    break;
                }
            }
            let val = num.parse().unwrap_or(0.0);
            return Some(Token::Number(val));
        }
        if ch == '"' {
            let mut s = String::new();
            while let Some(c) = self.next_char() {
                if c == '"' {
                    break;
                }
                s.push(c);
            }
            return Some(Token::String(s));
        }
        Some(Token::Symbol(ch))
    }

    pub fn tokenize(input: &'a str) -> Vec<Token> {
        let mut tokenizer = Self::new(input);
        let mut tokens = Vec::new();
        while let Some(tok) = tokenizer.next_token() {
            tokens.push(tok);
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_basic() {
        let tokens = Tokenizer::tokenize("foo 123 \"bar\" +");
        assert_eq!(
            tokens,
            vec![
                Token::Ident("foo".into()),
                Token::Number(123.0),
                Token::String("bar".into()),
                Token::Symbol('+'),
            ]
        );
    }
}
