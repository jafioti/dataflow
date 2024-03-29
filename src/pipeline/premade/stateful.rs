use crate::pipeline::Node;
use std::marker::PhantomData;

pub struct Stateful<I, O, S, F: Fn(I, &mut S) -> O, R: Fn(usize) -> usize> {
    _phantom: PhantomData<(I, O)>,
    function: F,
    state: S,
    remaining: R,
}

impl<I, O, S: Clone, F: Fn(I, &mut S) -> O + Clone, R: Fn(usize) -> usize + Clone> Clone
    for Stateful<I, O, S, F, R>
{
    fn clone(&self) -> Self {
        Self {
            _phantom: self._phantom,
            function: self.function.clone(),
            state: self.state.clone(),
            remaining: self.remaining.clone(),
        }
    }
}

fn identity_remaining(before: usize) -> usize {
    before
}

impl<I, O, S, F: Fn(I, &mut S) -> O> Stateful<I, O, S, F, fn(usize) -> usize> {
    /// Initialize a new stateful node, with a state and a process function.
    pub fn new(state: S, function: F) -> Self {
        Stateful {
            _phantom: PhantomData::default(),
            function,
            state,
            remaining: identity_remaining,
        }
    }
}

impl<I, O, S, F: Fn(I, &mut S) -> O, R: Fn(usize) -> usize> Stateful<I, O, S, F, R> {
    pub fn remaining<N: Fn(usize) -> usize>(self, remaining_fn: N) -> Stateful<I, O, S, F, N> {
        Stateful {
            _phantom: PhantomData::default(),
            function: self.function,
            state: self.state,
            remaining: remaining_fn,
        }
    }
}

impl<I, O, S, F: Fn(I, &mut S) -> O, R: Fn(usize) -> usize> Node<I> for Stateful<I, O, S, F, R> {
    type Output = O;

    fn process(&mut self, input: I) -> Self::Output {
        (self.function)(input, &mut self.state)
    }

    fn data_remaining(&self, before: usize) -> usize {
        (self.remaining)(before)
    }
}
