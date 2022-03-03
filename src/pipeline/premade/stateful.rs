use std::marker::PhantomData;
use super::super::Node;

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
}

impl <I, O, S, F: Fn(Vec<I>, &mut S) -> Vec<O>>Node for Stateful<I, O, S, F> {
    type Input = I;
    type Output = O;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        (self.function)(input, &mut self.state)
    }
}