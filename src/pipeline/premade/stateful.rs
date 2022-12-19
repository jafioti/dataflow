use crate::pipeline::ExplicitNode;

use super::super::Node;
use std::marker::PhantomData;

pub struct Stateful<I, O, S, F: Fn(I, &mut S) -> O> {
    _phantom: PhantomData<(I, O)>,
    function: F,
    state: S,
}

impl<I, O, S, F: Fn(I, &mut S) -> O> Stateful<I, O, S, F> {
    /// Initialize a new stateful node, with a state and a process function.
    pub fn new(state: S, function: F) -> Self {
        Stateful {
            _phantom: PhantomData::default(),
            function,
            state,
        }
    }
}

impl<I, O, S, F: Fn(I, &mut S) -> O> Node for Stateful<I, O, S, F> {
    type Input = I;
    type Output = O;

    fn process(&mut self, input: Self::Input) -> Self::Output {
        (self.function)(input, &mut self.state)
    }
}

impl<I, O, S, F: Fn(I, &mut S) -> O> ExplicitNode<I, O> for Stateful<I, O, S, F> {
    fn process(&mut self, input: I) -> O {
        (self.function)(input, &mut self.state)
    }
}