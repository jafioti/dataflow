// TOKENIZATION TESTS
use crate::tokenization::*;

#[test]
fn tokenize_alphabet() {
    let letters: Vec<String> = vec!["h", "e", "l", "l", "o"].iter().map(|t| {(*t).to_string()}).collect();
    let tokenizer = AlphabetTokenizer::load();
    assert_eq!(tokenizer.tokenize("hello"), letters);
}

#[test]
fn tokenize_spaces() {
    let tokens: Vec<String> = vec!["hello", "how", "are", "you"].iter().map(|t| {(*t).to_string()}).collect();
    let tokenizer = WhitespaceTokenizer::load();
    assert_eq!(tokenizer.tokenize("hello how are you"), tokens);
}

#[test]
fn tokenize_sentences() {
    let tokens: Vec<String> = vec!["hello, how are you?", "good, how are you?"].iter().map(|t| {(*t).to_string()}).collect();
    let tokenizer = SentenceTokenizer::load();
    assert_eq!(tokenizer.tokenize("hello, how are you? good, how are you?"), tokens);
}

#[test]
fn tokenize_bpe() {
    let tokens: Vec<String> = vec!["hello", ",", " ", "how", " ", "are", " ", "you"].iter().map(|str| {str.to_string()}).collect();
    let tokenizer = BPETokenizer::load();
    assert_eq!(tokenizer.batch_tokenize(vec!["hello, how are you".to_string()]), vec![tokens.clone()]);
    assert_eq!(tokenizer.tokenize("hello, how are you"), tokens);
}

#[test]
fn tokenize_wordpiece() {
    let tokens: Vec<String> = vec!["hello", ",", "how", "are", "you"].iter().map(|str| {str.to_string()}).collect();
    let tokenizer = WordpieceTokenizer::load();
    assert_eq!(tokenizer.batch_tokenize(vec!["hello, how are you".to_string()]), vec![tokens.clone()]);
    assert_eq!(tokenizer.tokenize("hello, how are you"), tokens);
}

#[test]
fn untokenize_bpe() {
    let sentence = "hello, how are you?";
    let tokenizer = BPETokenizer::load();
    let tokens = tokenizer.tokenize(sentence);
    assert_eq!(tokenizer.untokenize(tokens), sentence.to_string());
}

#[test]
fn untokenize_wordpiece() {
    let sentence = "hello, how are you?";
    let tokenizer = WordpieceTokenizer::load();
    let tokens = tokenizer.tokenize(sentence);
    assert_eq!(tokenizer.untokenize(tokens), sentence.to_string());
}

#[test]
fn untokenize_alphabet() {
    let sentence = "hello, how are you?";
    let tokenizer = AlphabetTokenizer::load();
    let tokens = tokenizer.tokenize(sentence);
    assert_eq!(tokenizer.untokenize(tokens), sentence.to_string());
}

#[test]
fn untokenize_spaces() {
    let sentence = "hello, how are you?";
    let tokenizer = WhitespaceTokenizer::load();
    let tokens = tokenizer.tokenize(sentence);
    assert_eq!(tokenizer.untokenize(tokens), sentence.to_string());
}

#[test]
fn untokenize_sentences() {
    let sentence = "hello how are you? good, how are you?";
    let tokenizer = WhitespaceTokenizer::load();
    let tokens = tokenizer.tokenize(sentence);
    assert_eq!(tokenizer.untokenize(tokens), sentence.to_string());
}