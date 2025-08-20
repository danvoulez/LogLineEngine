use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::tokenizer::tokenizer::{Token, Tokenizer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loader;

impl Loader {
    pub fn from_str(input: &str) -> Vec<Token> {
        Tokenizer::tokenize(input)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Token>> {
        let content = fs::read_to_string(path)?;
        Ok(Tokenizer::tokenize(&content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn load_from_str() {
        let tokens = Loader::from_str("foo");
        assert_eq!(tokens, vec![Token::Ident("foo".into())]);
    }

    #[test]
    fn load_from_file() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "foo").unwrap();
        let tokens = Loader::from_file(file.path()).unwrap();
        assert_eq!(tokens, vec![Token::Ident("foo".into())]);
    }
}
