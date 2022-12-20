#![allow(clippy::type_complexity)]

use std::marker::PhantomData;

use crate::pipeline::{ExplicitNode, Node, NodeContainer};

pub struct Map<I, O, N: Node<Input = I, Output = O>> {
    _phantom: PhantomData<(I, O)>,
    node: N,
}

impl<I, O, E: ExplicitNode<I, O>> Map<I, O, NodeContainer<I, O, E>> {
    pub fn new<T: Into<NodeContainer<I, O, E>>>(node: T) -> Self {
        Map {
            _phantom: PhantomData::default(),
            node: node.into(),
        }
    }
}

impl<I, O, N: Node<Input = I, Output = O>> ExplicitNode<Vec<I>, Vec<O>> for Map<I, O, N> {
    fn process(&mut self, input: Vec<I>) -> Vec<O> {
        input.into_iter().map(|i| self.node.process(i)).collect()
    }
}
