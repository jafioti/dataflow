use serde::{Serialize, Deserialize};
use super::{BasicVocab, TokenNotFoundError, Vocab};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct WordPieceVocab {
    vocab: BasicVocab
}

impl Vocab for WordPieceVocab {
    fn load() -> Self {
        // Open vocab file
        let tokens: Vec<&str> = include_str!("../resources/wordpiece_vocab.txt").split('\n').collect();
        // Build vocab
        let mut vocab = BasicVocab::new();
        for token in tokens {
            vocab.add_token(String::from(token));
        }
        WordPieceVocab{vocab}
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