use crate::pipeline::{Node, ExplicitNode};
use std::{cmp::Ordering, marker::PhantomData};

pub struct Sort<T, F: Fn(&T, &T) -> Ordering> {
    _phantom: PhantomData<T>,
    sort_fn: F,
}

impl<T, F: Fn(&T, &T) -> Ordering> Sort<T, F> {
    pub fn new(sort_fn: F) -> Self {
        Sort {
            _phantom: PhantomData::default(),
            sort_fn,
        }
    }
}

impl<T, F: Fn(&T, &T) -> Ordering> Node for Sort<T, F> {
    type Input = Vec<T>;
    type Output = Vec<T>;

    fn process(&mut self, mut input: Self::Input) -> Self::Output {
        input.sort_by(&self.sort_fn);
        input
    }
}

impl<T, F: Fn(&T, &T) -> Ordering> ExplicitNode<Vec<T>, Vec<T>> for Sort<T, F> {
    fn process(&mut self, mut input: Vec<T>) -> Vec<T> {
        input.sort_by(&self.sort_fn);
        input
    }
}