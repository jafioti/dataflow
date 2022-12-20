#[cfg(test)]
mod tests;

mod basic;
pub(crate) use basic::BasicVocab;
mod wordpiece;
pub use wordpiece::WordPieceVocab;
mod bpe;
pub use bpe::BPEVocab;

use serde::Serialize;

/// A trait every vocab object must implement
pub trait Vocab: Serialize + Default + Clone {
    fn load() -> Self;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn tokens_from_indexes(&self, indexes: &[usize]) -> Result<Vec<String>, TokenNotFoundError>;
    fn batch_tokens_from_indexes(
        &self,
        indexes: &[Vec<usize>],
    ) -> Result<Vec<Vec<String>>, TokenNotFoundError>;
    fn indexes_from_tokens(&self, tokens: &[String]) -> Result<Vec<usize>, TokenNotFoundError>;
    fn batch_indexes_from_tokens(
        &self,
        tokens: &[Vec<String>],
    ) -> Result<Vec<Vec<usize>>, TokenNotFoundError>;
}

/// Custom Error Types
#[derive(Debug)]
pub struct TokenNotFoundError;
