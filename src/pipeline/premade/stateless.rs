use crate::pipeline::{Node, ExplicitNode};
use std::marker::PhantomData;

pub struct Stateless<I, O, F: Fn(I) -> O> {
    _phantom: PhantomData<(I, O)>,
    function: F,
}

impl<I, O, F: Fn(I) -> O> Stateless<I, O, F> {
    pub fn new(function: F) -> Self {
        Stateless {
            _phantom: PhantomData::default(),
            function,
        }
    }
}

impl<I, O, F: Fn(I) -> O> ExplicitNode<I, O> for Stateless<I, O, F> {
    fn process(&mut self, input: I) -> O {
        (self.function)(input)
    }
}

impl<I, O, F: Fn(I) -> O> Node for Stateless<I, O, F> {
    type Input = I;
    type Output = O;

    fn process(&mut self, input: Self::Input) -> Self::Output {
        (self.function)(input)
    }
}

impl<I, O> From<fn(I) -> O> for Stateless<I, O, fn(I) -> O> {
    fn from(f: fn(I) -> O) -> Self {
        Stateless::new(f)
    }
}