use super::Tokenizer;

use dataflow::prelude::Node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlphabetTokenizer {}

impl Tokenizer for AlphabetTokenizer {
    fn load() -> Self {
        AlphabetTokenizer {}
    }

    fn tokenize(&self, string: &str) -> Vec<String> {
        let tokens: Vec<String> = string.split("").map(|f| f.to_string()).collect();
        tokens[1..tokens.len() - 1].to_vec() // For some reason, the split adds empty strings to each end
    }

    fn batch_tokenize(&self, strings: Vec<String>) -> Vec<Vec<String>> {
        strings
            .iter()
            .map(|string| string.split("").map(|f| f.to_string()).collect())
            .collect()
    }

    fn untokenize(&self, tokens: Vec<String>) -> String {
        tokens.join("")
    }

    fn batch_untokenize(&self, tokens: Vec<Vec<String>>) -> Vec<String> {
        tokens.iter().map(|tokens| tokens.join("")).collect()
    }
}

impl Node<String> for AlphabetTokenizer {
    type Output = Vec<String>;
    fn process(&mut self, input: String) -> Self::Output {
        self.tokenize(&input)
    }
}

impl Node<&str> for AlphabetTokenizer {
    type Output = Vec<String>;
    fn process(&mut self, input: &str) -> Self::Output {
        self.tokenize(input)
    }
}

impl Node<Vec<String>> for AlphabetTokenizer {
    type Output = Vec<Vec<String>>;

    fn process(&mut self, input: Vec<String>) -> Self::Output {
        self.batch_tokenize(input)
    }
}
