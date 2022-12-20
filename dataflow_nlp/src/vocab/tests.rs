use crate::vocab::{BPEVocab, Vocab, WordPieceVocab};
#[test]
fn creating_vocab() {
    let _wordpiece_vocab = WordPieceVocab::load();
    let _bpe_vocab = BPEVocab::load();
}

#[test]
fn indexes_from_tokens_bpe() {
    let bpe_vocab = BPEVocab::load();
    let tokens = ["Hello", ",", " ", "how", " ", "are", " ", "you", "?"];
    let mut tokens_vec: Vec<String> = Vec::new();
    for token in tokens.iter() {
        tokens_vec.push(String::from(*token));
    }
    let indexes = bpe_vocab.indexes_from_tokens(&tokens_vec);
    assert_eq!(
        indexes.unwrap(),
        vec![23858, 37861, 4, 4786, 4, 290, 4, 3258, 22092]
    );
}

#[test]
fn indexes_from_tokens_wordpiece() {
    let wordpiece_vocab = WordPieceVocab::load();
    let tokens = ["hello", ",", "how", "are", "you", "?"];
    let mut tokens_vec: Vec<String> = Vec::new();
    for token in tokens.iter() {
        tokens_vec.push(String::from(*token));
    }
    let indexes = wordpiece_vocab.indexes_from_tokens(&tokens_vec);
    assert_eq!(indexes.unwrap(), vec![7596, 1014, 2133, 2028, 2021, 1033]);
}

#[test]
fn tokens_from_indexes_bpe() {
    let bpe_vocab = BPEVocab::load();
    let tokens = ["Hello", ",", " ", "how", " ", "are", " ", "you", "?"];
    let mut tokens_vec: Vec<String> = Vec::new();
    for token in tokens.iter() {
        tokens_vec.push(String::from(*token));
    }
    let tokens = bpe_vocab.tokens_from_indexes(&[23858, 37861, 4, 4786, 4, 290, 4, 3258, 22092]);
    assert_eq!(tokens.unwrap(), tokens_vec);
}

#[test]
fn tokens_from_indexes_wordpiece() {
    let wordpiece_vocab = WordPieceVocab::load();
    let tokens = ["hello", ",", "how", "are", "you", "?"];
    let mut tokens_vec: Vec<String> = Vec::new();
    for token in tokens.iter() {
        tokens_vec.push(String::from(*token));
    }
    let tokens = wordpiece_vocab.tokens_from_indexes(&[7596, 1014, 2133, 2028, 2021, 1033]);
    assert_eq!(tokens.unwrap(), tokens_vec);
}

#[test]
fn batch_indexes_from_tokens() {
    let bpe_vocab = BPEVocab::load();
    let tokens = ["Hello", ",", " ", "how", " ", "are", " ", "you", "?"];
    let mut tokens_vec: Vec<Vec<String>> = vec![Vec::new()];
    for token in tokens.iter() {
        tokens_vec[0].push(String::from(*token));
    }
    let indexes = bpe_vocab.batch_indexes_from_tokens(&tokens_vec);
    assert_eq!(
        indexes.unwrap()[0],
        vec![23858, 37861, 4, 4786, 4, 290, 4, 3258, 22092]
    );
}

#[test]
fn batch_tokens_from_indexes() {
    let bpe_vocab = BPEVocab::load();
    let tokens = ["Hello", ",", " ", "how", " ", "are", " ", "you", "?"];
    let mut tokens_vec: Vec<String> = Vec::new();
    for token in tokens.iter() {
        tokens_vec.push(String::from(*token));
    }
    let tokens =
        bpe_vocab.batch_tokens_from_indexes(&[vec![23858, 37861, 4, 4786, 4, 290, 4, 3258, 22092]]);
    assert_eq!(tokens.unwrap()[0], tokens_vec);
}
