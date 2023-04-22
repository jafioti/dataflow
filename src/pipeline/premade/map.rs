#![allow(clippy::type_complexity)]

use std::marker::PhantomData;

use crate::pipeline::Node;

pub struct Map<I, O, N: Node<I, Output = O>> {
    _phantom: PhantomData<(I, O)>,
    node: N,
}

impl<I, O, E: Node<I, Output = O>> Map<I, O, E> {
    pub fn new(node: E) -> Self {
        Map {
            _phantom: PhantomData::default(),
            node,
        }
    }
}

impl<I, O, N: Node<I, Output = O>> Node<Vec<I>> for Map<I, O, N> {
    type Output = Vec<O>;

    fn process(&mut self, input: Vec<I>) -> Self::Output {
        input.into_iter().map(|i| self.node.process(i)).collect()
    }
}
