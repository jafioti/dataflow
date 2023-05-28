use super::{BasicVocab, TokenNotFoundError, Vocab};
use dataflow::prelude::Node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct WordPieceVocab {
    vocab: BasicVocab,
}

impl Vocab for WordPieceVocab {
    fn load() -> Self {
        // Open vocab file
        let tokens: Vec<&str> = include_str!("../resources/wordpiece_vocab.txt")
            .split('\n')
            .collect();
        // Build vocab
        let mut vocab = BasicVocab::new();
        for token in tokens {
            vocab.add_token(String::from(token));
        }
        WordPieceVocab { vocab }
    }

    fn len(&self) -> usize {
        self.vocab.len()
    }

    fn tokens_from_indexes(&self, indexes: &[usize]) -> Result<Vec<String>, TokenNotFoundError> {
        self.vocab.tokens_from_indexes(indexes)
    }

    fn batch_tokens_from_indexes(
        &self,
        indexes: &[Vec<usize>],
    ) -> Result<Vec<Vec<String>>, TokenNotFoundError> {
        self.vocab.batch_tokens_from_indexes(indexes)
    }

    fn indexes_from_tokens(&self, tokens: &[String]) -> Result<Vec<usize>, TokenNotFoundError> {
        self.vocab.indexes_from_tokens(tokens)
    }

    fn batch_indexes_from_tokens(
        &self,
        tokens: &[Vec<String>],
    ) -> Result<Vec<Vec<usize>>, TokenNotFoundError> {
        self.vocab.batch_indexes_from_tokens(tokens)
    }
}

impl Node<Vec<Vec<String>>> for WordPieceVocab {
    type Output = Vec<Vec<usize>>;

    fn process(&mut self, input: Vec<Vec<String>>) -> Self::Output {
        self.batch_indexes_from_tokens(&input).unwrap()
    }
}

impl Node<Vec<String>> for WordPieceVocab {
    type Output = Vec<usize>;

    fn process(&mut self, input: Vec<String>) -> Self::Output {
        self.indexes_from_tokens(&input).unwrap()
    }
}

impl Node<Vec<Vec<usize>>> for WordPieceVocab {
    type Output = Vec<Vec<String>>;

    fn process(&mut self, input: Vec<Vec<usize>>) -> Self::Output {
        self.batch_tokens_from_indexes(&input).unwrap()
    }
}

impl Node<Vec<usize>> for WordPieceVocab {
    type Output = Vec<String>;

    fn process(&mut self, input: Vec<usize>) -> Self::Output {
        self.tokens_from_indexes(&input).unwrap()
    }
}
