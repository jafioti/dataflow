use super::Tokenizer;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SentenceTokenizer {
    keep_punctuation: bool,
}

impl SentenceTokenizer {
    pub fn new(keep_punctuation: bool) -> Self {
        SentenceTokenizer { keep_punctuation }
    }
}

impl Tokenizer for SentenceTokenizer {
    fn load() -> Self {
        SentenceTokenizer {
            keep_punctuation: true,
        }
    }

    fn tokenize(&self, string: &str) -> Vec<String> {
        if self.keep_punctuation {
            split_keep(string)
                .into_iter()
                .map(|i| i.trim().to_string())
                .collect()
        } else {
            let regex = Regex::new(r"!|.|\?|;").unwrap();
            regex.split(string).map(|i| i.trim().to_string()).collect()
        }
    }

    fn batch_tokenize(&self, strings: Vec<String>) -> Vec<Vec<String>> {
        let regex = Regex::new(r"!|.|\?|;").unwrap();
        if self.keep_punctuation {
            strings
                .iter()
                .map(|string| {
                    split_keep(string)
                        .into_iter()
                        .map(|i| i.trim().to_string())
                        .collect()
                })
                .collect()
        } else {
            strings
                .iter()
                .map(|string| regex.split(string).map(|i| i.trim().to_string()).collect())
                .collect()
        }
    }

    fn untokenize(&self, tokens: Vec<String>) -> String {
        tokens.join(" ")
    }

    fn batch_untokenize(&self, tokens: Vec<Vec<String>>) -> Vec<String> {
        tokens.iter().map(|tokens| tokens.join(" ")).collect()
    }
}

fn split_keep(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let chars = text.chars();
    let mut last_match = 0;
    for (i, char) in chars.enumerate() {
        if char == '!' || char == '.' || char == '?' || char == ';' {
            // If we have more than one letter that came before, add it to the results
            if i - last_match > 0 {
                result.push(&text[last_match..i + 1]);
                last_match = i + 1;
            }
        }
    }
    if last_match < text.len() - 1 {
        result.push(&text[last_match..]);
    }
    result
}
