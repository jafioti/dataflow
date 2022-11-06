// BATCHING TESTS
use crate::batching;

#[test]
fn pad_mask_test() {
    let batch = vec![vec!["d", "hello", "how"], vec!["hi", "yo", "PAD"]];
    let expected_mask = vec![vec![false, false, false], vec![false, false, true]];
    let pad_mask = batching::pad_mask(&batch, "PAD");
    assert_eq!(expected_mask, pad_mask);
}

#[test]
fn pad_batch_test() {
    let mut seqs = vec![vec![1, 2, 3, 1], vec![1, 4, 6, 2, 3, 5, 67]];
    let expected_padded_batch = vec![vec![1, 2, 3, 1, 0, 0, 0], vec![1, 4, 6, 2, 3, 5, 67]];
    batching::pad_batch(&mut seqs, 0);
    assert_eq!(seqs, expected_padded_batch);
}

#[test]
fn filter_by_length_test() {
    let mut seqs = vec![
        vec![vec![1, 2, 3, 1], vec![1, 4, 6, 2, 3, 5, 67], vec![1, 2, 3]],
        vec![vec![1, 1], vec![1, 67], vec![1, 2, 3]],
    ];
    let expected_seqs = vec![
        vec![vec![1, 2, 3, 1], vec![1, 2, 3]],
        vec![vec![1, 1], vec![1, 2, 3]],
    ];
    batching::filter_by_length(&mut seqs, None, Some(6));
    assert_eq!(seqs, expected_seqs);
}

#[test]
fn shuffle_lists_test() {
    let mut seqs = vec![
        vec![vec![1, 2, 3, 1], vec![1, 4, 6, 2, 3, 5, 67], vec![1, 2, 3]],
        vec![vec![1, 1], vec![1, 67], vec![1, 2, 3]],
    ];
    let orig_seqs = seqs.clone();
    for _ in 0..10 {
        batching::shuffle_lists(&mut seqs);
        if seqs != orig_seqs {
            break;
        }
    }
    assert_ne!(seqs, orig_seqs);
}

#[test]
fn sort_lists_by_length_test() {
    let mut seqs = vec![
        vec![
            "hello".to_string(),
            "how are you".to_string(),
            "yo".to_string(),
        ],
        vec!["hey".to_string(), "wow".to_string(), "who".to_string()],
    ];
    let sorted_seqs = vec![
        vec![
            "yo".to_string(),
            "hello".to_string(),
            "how are you".to_string(),
        ],
        vec!["who".to_string(), "hey".to_string(), "wow".to_string()],
    ];
    let reverse_sorted_seqs = vec![
        vec![
            "how are you".to_string(),
            "hello".to_string(),
            "yo".to_string(),
        ],
        vec!["wow".to_string(), "hey".to_string(), "who".to_string()],
    ];
    batching::sort_lists_by_length(&mut seqs, Some(false));
    assert_eq!(seqs, sorted_seqs);
    batching::sort_lists_by_length(&mut seqs, Some(true));
    assert_eq!(seqs, reverse_sorted_seqs);
}
