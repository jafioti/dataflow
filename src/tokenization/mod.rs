#[cfg(test)]
mod tests;
use std::fmt::Debug;

// Tokenizers
mod wordpiece;
pub use wordpiece::WordpieceTokenizer;
mod bpe;
pub use bpe::BPETokenizer;
mod whitespace;
pub use whitespace::WhitespaceTokenizer;
mod alphabet;
pub use alphabet::AlphabetTokenizer;
mod sentence;
pub use sentence::SentenceTokenizer;

/// A trait to implement for all tokenizers, contains basic tokenizing and untokenizing functions
pub trait Tokenizer: Debug + Send + Sync {
    /// Load the tokenizer
    fn load() -> Self;
    /// Tokenize a single string
    fn tokenize(&self, string: &str) -> Vec<String>;
    /// Tokenize a batch of strings
    fn batch_tokenize(&self, strings: Vec<String>) -> Vec<Vec<String>>;
    /// Untokenize a single string
    fn untokenize(&self, tokens: Vec<String>) -> String;
    /// Untokenize a batch of strings
    fn batch_untokenize(&self, tokens: Vec<Vec<String>>) -> Vec<String>;
}
