use crate::pipeline::Node;
use std::{cmp::Ordering, marker::PhantomData};

pub struct Sort<T, F: Fn(&T, &T) -> Ordering> {
    _phantom: PhantomData<T>,
    sort_fn: F,
}

impl<T, F: Clone + Fn(&T, &T) -> Ordering> Clone for Sort<T, F> {
    fn clone(&self) -> Self {
        Self {
            _phantom: self._phantom,
            sort_fn: self.sort_fn.clone(),
        }
    }
}

impl<T, F: Fn(&T, &T) -> Ordering> Sort<T, F> {
    pub fn new(sort_fn: F) -> Self {
        Sort {
            _phantom: PhantomData::default(),
            sort_fn,
        }
    }
}

impl<T, F: Fn(&T, &T) -> Ordering> Node<Vec<T>> for Sort<T, F> {
    type Output = Vec<T>;

    fn process(&mut self, mut input: Vec<T>) -> Self::Output {
        input.sort_by(&self.sort_fn);
        input
    }
}
