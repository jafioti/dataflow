use std::{marker::PhantomData, cmp::Ordering};
use crate::pipeline::Node;

pub struct Sort<T, F: Fn(&T, &T) -> Ordering> {
    _phantom: PhantomData<T>,
    sort_fn: F
}

impl <T, F: Fn(&T, &T) -> Ordering>Sort<T, F> {
    pub fn new(sort_fn: F) -> Self {
        Sort { 
            _phantom: PhantomData::default(), 
            sort_fn
        }
    }
}

impl <T, F: Fn(&T, &T) -> Ordering>Node for Sort<T, F> {
    type Input = T;
    type Output = T;

    fn process(&mut self, mut input: Vec<Self::Input>) -> Vec<Self::Output> {
        input.sort_by(&self.sort_fn);
        input
    }
}