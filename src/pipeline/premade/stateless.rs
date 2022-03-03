use std::marker::PhantomData;
use crate::pipeline::Node;

pub struct BatchStateless<I, O, F: Fn(Vec<I>) -> Vec<O>> {
    _phantom: PhantomData<(I, O)>,
    function: F
}

impl <I, O, F: Fn(Vec<I>) -> Vec<O>>BatchStateless<I, O, F> {
    pub fn new(function: F) -> Self {
        BatchStateless {
            _phantom: PhantomData::default(),
            function
        }
    }
}

impl <I, O, F: Fn(Vec<I>) -> Vec<O>>Node for BatchStateless<I, O, F> {
    type Input = I;
    type Output = O;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        (self.function)(input)    
    }
}

/// Really just so that we can use add_fn() on the node, there must be a better way than creating a node just for this
pub struct SingleStateless<I, O, F: Fn(I) -> O> {
    _phantom: PhantomData<(I, O)>,
    function: F
}

impl <I, O, F: Fn(I) -> O>SingleStateless<I, O, F> {
    pub fn new(function: F) -> Self {
        SingleStateless {
            _phantom: PhantomData::default(),
            function
        }
    }
}

impl <I, O, F: Fn(I) -> O>Node for SingleStateless<I, O, F> {
    type Input = I;
    type Output = O;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        input.into_iter().map(&self.function).collect()  
    }
}