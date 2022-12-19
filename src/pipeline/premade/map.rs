#![allow(clippy::type_complexity)]

use std::marker::PhantomData;

use crate::pipeline::{Node, Connector, NodeContainer, ExplicitNode};

pub struct Map<I, O, N: Node<Input=I, Output=O>> {
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

impl<I, O, N: Node<Input=I, Output=O>> ExplicitNode<Vec<I>, Vec<O>> for Map<I, O, N> {
    fn process(&mut self, input: Vec<I>) -> Vec<O> {
        input.into_iter().map(|i| self.node.process(i)).collect()
    }
}

pub trait MapTrait<I, O>: Sized + Node<Output = Vec<I>> {
    fn map<E: ExplicitNode<I, O>, T: Into<NodeContainer<I, O, E>>>(self, node: T) -> Connector<Self, NodeContainer<Vec<I>, Vec<O>, Map<I, O, NodeContainer<I, O, E>>>> {
        Connector {
            node1: self,
            node2: Map::new(node).into(),
        }
    }
}

impl <I, O, N> MapTrait<I, O> for N
where N: Node<Output = Vec<I>> + Sized {}