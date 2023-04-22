#![allow(clippy::type_complexity)]

use std::marker::PhantomData;

use crate::pipeline::Node;

pub struct Map<I, N: Node<I>> {
    _phantom: PhantomData<I>,
    node: N,
}

impl<I, O, E: Node<I, Output = O>> Map<I, E> {
    pub fn new(node: E) -> Self {
        Map {
            _phantom: PhantomData::default(),
            node,
        }
    }
}

impl<I, O, N: Node<I, Output = O>> Node<Vec<I>> for Map<I, N> {
    type Output = Vec<O>;

    fn process(&mut self, input: Vec<I>) -> Self::Output {
        input.into_iter().map(|i| self.node.process(i)).collect()
    }
}
