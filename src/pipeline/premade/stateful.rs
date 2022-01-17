use std::marker::PhantomData;
use super::super::{Connector, Node, Stateless};

pub struct Stateful<I, O, S, F: Fn(Vec<I>, &mut S) -> Vec<O>> {
    _phantom: PhantomData<(I, O)>,
    function: F,
    state: S
}

impl <I, O, S, F: Fn(Vec<I>, &mut S) -> Vec<O>>Stateful<I, O, S, F> {
    pub fn new(function: F, state: S) -> Self {
        Stateful {
            _phantom: PhantomData::default(),
            function,
            state
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

impl <I, O, S, F: Fn(Vec<I>, &mut S) -> Vec<O>>Node for Stateful<I, O, S, F> {
    type Input = I;
    type Output = O;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        (self.function)(input, &mut self.state)
    }
}