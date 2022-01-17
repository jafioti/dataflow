use std::marker::PhantomData;
use super::super::{Connector, Node};

pub struct Stateless<I, O, F: Fn(Vec<I>) -> Vec<O>> {
    _phantom: PhantomData<(I, O)>,
    function: F
}

impl <I, O, F: Fn(Vec<I>) -> Vec<O>>Stateless<I, O, F> {
    pub fn new(function: F) -> Self {
        Stateless {
            _phantom: PhantomData::default(),
            function
        }
    }

    pub fn add_node<N: Node<Input = O>>(self, node: N) -> Connector<Self, N> {
        Connector::new(self, node)
    }

    pub fn add_fn<NO, F1: Fn(Vec<O>) -> Vec<NO>>(self, function: F1) -> Connector<Self, Stateless<O, NO, F1>> {
        Connector::new(
            self,
            Stateless::new(function)
        )
    }
}

impl <I, O, F: Fn(Vec<I>) -> Vec<O>>Node for Stateless<I, O, F> {
    type Input = I;
    type Output = O;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        (self.function)(input)    
    }
}