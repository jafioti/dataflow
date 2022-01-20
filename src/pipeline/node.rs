use super::{Connector, Duplicator, Pair, Stateless};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub trait Node {
    type Input;
    type Output;
    
    /// Process a batch of data
    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output>;
    /// Reset signal propogates through pipeline
    fn reset(&mut self) {}
    /// Get number of examples left
    fn data_remaining(&self) -> usize {usize::MAX} // Defaults to max for non-source functions

    fn add_node<N: Node<Input = Self::Output>>(self, node: N) -> Connector<Self, N> where Self: std::marker::Sized {
        Connector::new(self, node)
    }

    /// Add function to pipeline
    fn add_fn<O, F: Fn(Vec<Self::Output>) -> Vec<O>>(self, function: F) -> Connector<Self, Stateless<Self::Output, O, F>> 
    where Self: std::marker::Sized {
        Connector::new(
            self,
            Stateless::new(function)
        )
    }

    // /// Add function that takes a single datapoint and outputs a single datapoint
    // fn add_single_fn<O, F: Fn(Self::Output) -> O + Send + Sync, B: Fn(Vec<Self::Output>) -> Vec<O>>(self, function: F) -> Connector<Self, Stateless<Self::Output, O, B>> 
    // where Self: std::marker::Sized, 
    // <Self as crate::pipeline::node::Node>::Output: std::marker::Sized, 
    // [<Self as crate::pipeline::node::Node>::Output]: rayon::iter::ParallelIterator {
    //     Connector::new(
    //         self,
    //         Stateless::new(|input: Vec<Self::Output>| {
    //             input.into_par_iter().map(function).collect()
    //         })
    //     )
    // }

    #[allow(clippy::type_complexity)]
    fn split<N3: Node<Input = Self::Output>, N4: Node<Input = Self::Output>>(self, node1: N3, node2: N4) -> Connector<Connector<Self, Duplicator<Self::Output>>, Pair<N3, N4>> 
    where Self: std::marker::Sized, Self::Output: Clone {
        Connector::new(
            Connector::new(self, Duplicator::default()),
            Pair::new(node1, node2)
        )
    }

    fn pair<O1, O2, N3: Node<Input = O1>, N4: Node<Input = O2>>(self, node1: N3, node2: N4) -> Connector<Self, Pair<N3, N4>> 
    where Self: std::marker::Sized, Self: Node<Output = (O1, O2)>{
        Connector::new(
            self,
            Pair::new(node1, node2)
        )
    }
}

// Attempt at implementing node for every function
// impl <I, O, F: Fn(I) -> O> Node for F {
//     type Input = I;
//     type Output = O;
//     fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
//         input.into_iter().map(self).collect()
//     }
// }

// Current implementation of node for every function pointer, requires an ugly cast to use though
impl <I, O> Node for fn(Vec<I>) -> Vec<O> {
    type Input = I;
    type Output = O;
    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        (self)(input)
    }
}