#[cfg(test)]
mod tests;

use lentrait::Len;

/// Create a pad mask based on the values in the batch (batch shape: batch size, seq len)
pub fn pad_mask<T: std::cmp::PartialEq>(batch: &[Vec<T>], pad_value: T) -> Vec<Vec<bool>> {
    let mut mask: Vec<Vec<bool>> = vec![Vec::with_capacity(batch[0].len()); batch.len()];
    for (i, seq)in batch.iter().enumerate() {
        for token in seq {
            mask[i].push(*token == pad_value);
        }
    }
    mask
}

/// Pad all sequences to the length of the longest sequence
pub fn pad_batch<T: std::clone::Clone>(batch: &mut [Vec<T>], pad_value: T) {
    // Get longest example
    let mut longest = 0;
    for example in batch.iter() {
        if example.len() > longest {longest = example.len();}
    }

    // Pad all sequences to be longest
    for example in batch.iter_mut() {
        while example.len() < longest {
            example.push(pad_value.clone());
        }
    }
}

/// Filters lists by a max length and returns the ones under the max
pub fn filter_by_length<T: Len> (lists: &mut [Vec<T>], min_length: Option<usize>, max_length: Option<usize>) {
    // Loop through elements in all lists
    for i in (0..lists[0].len()).rev() {
        // Loop through each list
        for x in 0..lists.len() {
            // If element length is greater than max_length or less than min_length, remove element i from every list
            if lists[x][i].len() > max_length.unwrap_or(usize::MAX) || lists[x][i].len() < min_length.unwrap_or(0) {
                for list in lists.iter_mut() {
                    list.remove(i);
                }
                break;
            }
        }
    }
}

/// Shuffles multiple lists of the same length in the same ways
pub fn shuffle_lists<T: std::clone::Clone>(lists: &mut [Vec<T>]) {
    use rand::thread_rng;
    use rand::seq::SliceRandom;

    // Zip lists
    let mut zipped: Vec<Vec<T>> = vec![Vec::with_capacity(lists.len()); lists[0].len()];
    for list in lists.iter() {
        for (i, item) in list.iter().enumerate() {
            zipped[i].push(item.clone());
        }
    }
    // Shuffle
    zipped.shuffle(&mut thread_rng());
    // Unzip lists
    for (x, list) in lists.iter_mut().enumerate() {
        for (i, item) in list.iter_mut().enumerate() {
            *item = zipped[i][x].clone();
        }
    }
}

/// Sort lists by length. Uses the lengths of the elements in the first list passed in
pub fn sort_lists_by_length<T: Len + std::clone::Clone>(lists: &mut [Vec<T>], longest_first:Option<bool>) {
    for i in 1..lists.len() {assert!(lists[i].len() == lists[0].len())} // Ensure all lists are the same length
    
    // Zip lists
    let mut zipped: Vec<Vec<T>> = vec![Vec::with_capacity(lists.len()); lists[0].len()];
    for list in lists.iter() {
        for (i, item) in list.iter().enumerate() {
            zipped[i].push(item.clone());
        }
    }
    // Sort lists
    zipped.sort_unstable_by(|a, b| {
        a[0].len().partial_cmp(&b[0].len()).expect("NaN found in lengths!")
    });
    // Reverse if longest first
    if longest_first.unwrap_or(false) {zipped.reverse()}
    // Unzip lists
    for (x, list) in lists.iter_mut().enumerate() {
        for (i, item) in list.iter_mut().enumerate() {
            *item = zipped[i][x].clone();
        }
    }
}