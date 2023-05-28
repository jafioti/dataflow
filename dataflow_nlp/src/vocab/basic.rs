use super::TokenNotFoundError;
use dataflow::prelude::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The basic vocab type used internally in WordpieceVocab and BPEVocab
#[derive(Serialize, Deserialize, Default, Clone)]
pub(crate) struct BasicVocab {
    pub num_tokens: usize,
    pub token2index: HashMap<String, usize>,
    pub index2token: Vec<String>,
    pub pad_token: usize,
    pub sos_token: usize,
    pub eos_token: usize,
    pub sep_token: usize,
}

impl BasicVocab {
    /// Make a new vocab
    pub fn new() -> Self {
        let mut voc = BasicVocab {
            num_tokens: 0,
            token2index: HashMap::new(),
            index2token: Vec::new(),
            pad_token: 0,
            sos_token: 1,
            eos_token: 2,
            sep_token: 3,
        };
        voc.add_tokens(vec![
            "[PAD]".to_string(),
            "[SOS]".to_string(),
            "[EOS]".to_string(),
            "[SEP]".to_string(),
        ]);
        voc
    }

    /// Returns num_tokens
    pub fn len(&self) -> usize {
        self.num_tokens
    }

    /// Add token to vocab
    pub fn add_token(&mut self, token: String) {
        self.token2index.insert(token.clone(), self.num_tokens);
        self.index2token.push(token);
        self.num_tokens += 1;
    }

    /// Add a vec of tokens to vocab
    pub fn add_tokens(&mut self, tokens: Vec<String>) {
        self.index2token.extend(tokens.clone());
        for (i, token) in tokens.iter().enumerate() {
            // Probably a more efficient way to do this and avoid the loop
            self.token2index.insert(token.clone(), self.num_tokens + i);
        }
        self.num_tokens += tokens.len();
    }

    /// Remove a vec of tokens from vocab (NOT SURE IF THIS SHOULD BE KEPT)
    pub fn _remove_tokens(&mut self, tokens: Vec<&String>) {
        for token in tokens {
            if self.token2index.contains_key(token) {
                self._remove_token(token);
            }
        }
    }

    /// Remove token from vocab
    pub fn _remove_token(&mut self, token: &str) {
        // Loop through all higher token2index mappings and decrement (must be a more efficient way to do this)
        for i in (self.token2index[token]) + 1..self.index2token.len() {
            *self.token2index.get_mut(&self.index2token[i]).unwrap() -= 1;
        }
        self.index2token.remove(self.token2index[token]);
        self.token2index.remove(token);
        self.num_tokens -= 1;
    }

    /// Get vec of tokens from vec of indexes
    pub fn tokens_from_indexes(
        &self,
        indexes: &[usize],
    ) -> Result<Vec<String>, TokenNotFoundError> {
        if *indexes.iter().max().unwrap() >= self.num_tokens {
            return Err(TokenNotFoundError);
        } // Make sure we aren't trying to get an index too big

        let mut tokens: Vec<String> = Vec::with_capacity(indexes.len());
        for index in indexes {
            tokens.push(self.index2token[*index].to_string());
        }
        Ok(tokens)
    }

    /// Batched version of tokens_from_indexes
    pub fn batch_tokens_from_indexes(
        &self,
        indexes: &[Vec<usize>],
    ) -> Result<Vec<Vec<String>>, TokenNotFoundError> {
        let mut tokens: Vec<Vec<String>> = Vec::with_capacity(indexes.len());
        for sent in indexes {
            tokens.push(self.tokens_from_indexes(sent)?);
        }
        Ok(tokens)
    }

    /// Get vec of indexes from vec of tokens
    pub fn indexes_from_tokens(&self, tokens: &[String]) -> Result<Vec<usize>, TokenNotFoundError> {
        let mut indexes: Vec<usize> = Vec::with_capacity(tokens.len());
        for token in tokens {
            indexes.push(match self.token2index.get(token) {
                Some(index) => *index,
                None => {
                    return Err(TokenNotFoundError);
                }
            });
        }
        Ok(indexes)
    }

    /// Batched version of indexes_from_tokens
    pub fn batch_indexes_from_tokens(
        &self,
        tokens: &[Vec<String>],
    ) -> Result<Vec<Vec<usize>>, TokenNotFoundError> {
        let mut indexes: Vec<Vec<usize>> = Vec::with_capacity(tokens.len());
        for sent in tokens {
            indexes.push(self.indexes_from_tokens(sent)?);
        }
        Ok(indexes)
    }
}

impl Node<Vec<Vec<String>>> for BasicVocab {
    type Output = Vec<Vec<usize>>;

    fn process(&mut self, input: Vec<Vec<String>>) -> Self::Output {
        self.batch_indexes_from_tokens(&input).unwrap()
    }
}

impl Node<Vec<String>> for BasicVocab {
    type Output = Vec<usize>;

    fn process(&mut self, input: Vec<String>) -> Self::Output {
        self.indexes_from_tokens(&input).unwrap()
    }
}

impl Node<Vec<Vec<usize>>> for BasicVocab {
    type Output = Vec<Vec<String>>;

    fn process(&mut self, input: Vec<Vec<usize>>) -> Self::Output {
        self.batch_tokens_from_indexes(&input).unwrap()
    }
}

impl Node<Vec<usize>> for BasicVocab {
    type Output = Vec<String>;

    fn process(&mut self, input: Vec<usize>) -> Self::Output {
        self.tokens_from_indexes(&input).unwrap()
    }
}
