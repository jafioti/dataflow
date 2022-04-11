use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use super::{BasicVocab, TokenNotFoundError, Vocab};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct BPEVocab {
    vocab: BasicVocab
}

impl Vocab for BPEVocab {
    fn load() -> Self {
        use serde_json::Value;

        // Open vocab file
        let json: HashMap<String, Value>  = serde_json::from_str(&include_str!("../resources/bpe_vocab.json").replace('/', "").replace('Ġ', "")).expect("Error parsing BPE vocab file!");
        // Build sorted vector of tokens from hashmap
        let mut token_vec: Vec<String> = vec![String::from(""); 50265]; // Happen to know the largest index in the json is 50264, this is a bad system
        for token in json.keys() {
            token_vec[json[token].as_u64().unwrap() as usize] = token.clone();
        }
        // Build vocab
        let mut vocab = BasicVocab::new();
        let mut temp_vec: Vec<String> = Vec::new();
        for token in token_vec {
            if !token.is_empty() {
                vocab.add_token(token.clone());
                temp_vec.push(token);
            }
        }
        BPEVocab{vocab}
    }

    fn len(&self) -> usize {
        self.vocab.len()
    }

    fn tokens_from_indexes(&self, indexes: &[usize]) -> Result<Vec<String>, TokenNotFoundError> {
        self.vocab.tokens_from_indexes(indexes)
    }

    fn batch_tokens_from_indexes(&self, indexes: &[Vec<usize>]) -> Result<Vec<Vec<String>>, TokenNotFoundError> {
        self.vocab.batch_tokens_from_indexes(indexes)
    }

    fn indexes_from_tokens(&self, tokens: &[String]) -> Result<Vec<usize>, TokenNotFoundError> {
        self.vocab.indexes_from_tokens(tokens)
    }

    fn batch_indexes_from_tokens(&self, tokens: &[Vec<String>]) -> Result<Vec<Vec<usize>>, TokenNotFoundError> {
        self.vocab.batch_indexes_from_tokens(tokens)
    }
}