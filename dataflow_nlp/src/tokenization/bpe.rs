use std::collections::HashMap;

use super::Tokenizer;

use dataflow::pipeline::ExplicitNode;
use serde::{Deserialize, Serialize};
use tokenizers::Tokenizer as HFTokenizer;

#[derive(Serialize, Deserialize, Debug)]
pub struct BPETokenizer {
    hf_tokenizer: HFTokenizer,
}

impl Tokenizer for BPETokenizer {
    fn load() -> Self {
        use serde_json::Value;
        use tokenizers::models::bpe::BPE;
        // Create token2index map
        // Open vocab file
        let json: HashMap<String, Value> = serde_json::from_str(
            &include_str!("../resources/bpe_vocab.json")
                .replace('/', "")
                .replace('Ġ', ""),
        )
        .expect("Error parsing BPE vocab file!");
        // Build sorted vector of tokens from hashmap
        let mut token_vec: Vec<String> = vec![String::from(""); 50265]; // Happen to know the largest index in the json is 50264, this is a bad system
        for token in json.keys() {
            token_vec[json[token].as_u64().unwrap() as usize] = token.clone();
        }
        // Build vocab
        let mut token2index = HashMap::with_capacity(token_vec.len());
        for token in token_vec {
            if !token.is_empty() {
                token2index.insert(token.to_string(), token2index.len() as u32);
            }
        }
        // Create tokenizer
        let bpe_builder = BPE::builder();
        let mut merges: Vec<(String, String)> = Vec::new();
        let lines: Vec<&str> = include_str!("../resources/bpe_merges.txt")
            .split('\n')
            .collect();
        for line in lines {
            let line = String::from(line)
                .replace(['Ġ', '\n'], "")
                .replace("##", "");
            // Filter out junk
            if line.contains(' ') && !line.contains('#') {
                let line: Vec<&str> = line.split(' ').collect();
                // Make sure vocab contains both tokens and combined token
                if token2index.contains_key(&line[0].to_string())
                    && token2index.contains_key(&line[1].to_string())
                    && token2index.contains_key(&format!("{}{}", line[0], line[1]))
                {
                    merges.push((line[0].to_string(), line[1].to_string()));
                }
            }
        }

        let bpe_builder = bpe_builder.vocab_and_merges(token2index, merges);
        let bpe = bpe_builder
            .unk_token("[UNK]".into())
            .build()
            .expect("BPE Tokenizer failed to build!");

        BPETokenizer {
            hf_tokenizer: HFTokenizer::new(bpe),
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
            .expect("BPE tokenization failed!");
        // Convert back to strings
        let mut tokens: Vec<Vec<String>> = Vec::with_capacity(encodings.len());
        for encoding in encodings {
            tokens.push(encoding.get_tokens().to_vec());
        }
        tokens
    }

    fn untokenize(&self, tokens: Vec<String>) -> String {
        tokens.join("")
    }

    fn batch_untokenize(&self, tokens: Vec<Vec<String>>) -> Vec<String> {
        tokens.iter().map(|tokens| tokens.join("")).collect()
    }
}

impl ExplicitNode<String, Vec<String>> for BPETokenizer {
    fn process(&mut self, input: String) -> Vec<String> {
        self.tokenize(&input)
    }
}

impl ExplicitNode<&str, Vec<String>> for BPETokenizer {
    fn process(&mut self, input: &str) -> Vec<String> {
        self.tokenize(input)
    }
}

impl ExplicitNode<Vec<String>, Vec<Vec<String>>> for BPETokenizer {
    fn process(&mut self, input: Vec<String>) -> Vec<Vec<String>> {
        self.batch_tokenize(input)
    }
}
