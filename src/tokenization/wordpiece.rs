use crate::pipeline::ExplicitNode;

use super::Tokenizer;
use serde::{Deserialize, Serialize};
use tokenizers::Tokenizer as HFTokenizer;

#[derive(Serialize, Deserialize, Debug)]
pub struct WordpieceTokenizer {
    hf_tokenizer: HFTokenizer,
}

impl Tokenizer for WordpieceTokenizer {
    fn load() -> Self {
        tokenizers::utils::parallelism::set_parallelism(true);
        use std::collections::HashMap;
        use tokenizers::models::wordpiece::WordPiece;
        use tokenizers::pre_tokenizers::whitespace::Whitespace;
        // Build tokenizer
        let wordpiece_builder = WordPiece::builder();
        let lines: Vec<&str> = include_str!("../resources/wordpiece_vocab.txt")
            .split('\n')
            .collect();
        let mut hashmap: HashMap<String, u32> = HashMap::new();
        for (i, line) in lines.iter().enumerate() {
            hashmap.insert(line.to_string(), i as u32);
        }
        let wordpiece_builder = wordpiece_builder.vocab(hashmap);
        let wordpiece = wordpiece_builder
            .build()
            .expect("WordPiece Tokenizer failed to build!");

        let mut tokenizer = HFTokenizer::new(wordpiece);
        tokenizer.with_pre_tokenizer(Whitespace::default());
        WordpieceTokenizer {
            hf_tokenizer: tokenizer,
        }
    }

    fn tokenize(&self, string: &str) -> Vec<String> {
        tokenizers::utils::parallelism::set_parallelism(true);
        // Create tokenizer and tokenize
        let encoding = self
            .hf_tokenizer
            .encode(string, false)
            .expect("BPE tokenization failed!");
        // Convert back to string
        encoding.get_tokens().to_vec()
    }

    fn batch_tokenize(&self, strings: Vec<String>) -> Vec<Vec<String>> {
        tokenizers::utils::parallelism::set_parallelism(true);
        // Create tokenizer and tokenize
        let encodings = self
            .hf_tokenizer
            .encode_batch(strings, false)
            .expect("WordPiece tokenization failed!");
        // Convert back to strings
        let mut tokens: Vec<Vec<String>> = Vec::with_capacity(encodings.len());
        for encoding in encodings.iter() {
            tokens.push(encoding.get_tokens().to_vec());
        }
        tokens
    }

    fn untokenize(&self, tokens: Vec<String>) -> String {
        let mut untokenized_string = String::new();
        for (i, token) in tokens.iter().enumerate() {
            if *token != "[PAD]" && *token != "[EOS]" {
                if token.contains("##")
                    || [".", "?", "!", ",", "'", r#"""#]
                        .iter()
                        .any(|x| *x == token)
                    || i == 0
                {
                    untokenized_string =
                        format!("{}{}", untokenized_string, token.replace("##", ""))
                } else {
                    untokenized_string = format!("{} {}", untokenized_string, token)
                }
            }
        }
        untokenized_string
    }

    fn batch_untokenize(&self, tokens: Vec<Vec<String>>) -> Vec<String> {
        let mut untokenized_strings = vec![String::new(); tokens.len()];
        for i in 0..tokens.len() {
            for x in 0..tokens[i].len() {
                if *tokens[i][x] != *"[PAD]" && *tokens[i][x] != *"[EOS]" {
                    if tokens[i][x].contains("##")
                        || [".", "?", "!", ",", "'", r#"""#]
                            .iter()
                            .any(|t| **t == tokens[i][x])
                        || x == 0
                    {
                        untokenized_strings[i] = format!(
                            "{}{}",
                            untokenized_strings[i],
                            tokens[i][x].replace("##", "")
                        )
                    } else {
                        untokenized_strings[i] =
                            format!("{} {}", untokenized_strings[i], tokens[i][x])
                    }
                }
            }
        }
        untokenized_strings
    }
}

impl ExplicitNode<String, Vec<String>> for WordpieceTokenizer {
    fn process(&mut self, input: String) -> Vec<String> {
        self.tokenize(&input)
    }
}

impl ExplicitNode<&str, Vec<String>> for WordpieceTokenizer {
    fn process(&mut self, input: &str) -> Vec<String> {
        self.tokenize(input)
    }
}

impl ExplicitNode<Vec<String>, Vec<Vec<String>>> for WordpieceTokenizer {
    fn process(&mut self, input: Vec<String>) -> Vec<Vec<String>> {
        self.batch_tokenize(input)
    }
}