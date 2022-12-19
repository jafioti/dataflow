use std::marker::PhantomData;

use super::{Connector, Duplicator, Pair};

pub trait Node {
    type Input;
    type Output;

    /// Process a batch of data
    fn process(&mut self, input: Self::Input) -> Self::Output;
    /// Reset signal propogates through pipeline
    fn reset(&mut self) {}
    /// Get number of examples left
    fn data_remaining(&self, before: usize) -> usize {before} // Defaults to same as previous remaining data

    fn node<O, N: ExplicitNode<Self::Output, O>, T: Into<NodeContainer<Self::Output, O, N>>>(self, node: T) -> Connector<Self, NodeContainer<Self::Output, O, N>>
    where
        Self: std::marker::Sized,
    {
        Connector {
            node1: self,
            node2: node.into()
        }
    }

    #[allow(clippy::type_complexity)]
    fn split<N3: Node<Input = Self::Output>, N4: Node<Input = Self::Output>>(
        self,
        node1: N3,
        node2: N4,
    ) -> Connector<Connector<Self, Duplicator<Self::Output>>, Pair<N3, N4>>
    where
        Self: std::marker::Sized,
        Self::Output: Clone,
    {
        Connector {
            node1: Connector {
                node1: self, 
                node2: Duplicator::default()
            },
            node2: Pair::new(node1, node2),
        }
    }

    fn pair<O1, O2, N3: Node<Input = O1>, N4: Node<Input = O2>>(
        self,
        node1: N3,
        node2: N4,
    ) -> Connector<Self, Pair<N3, N4>>
    where
        Self: std::marker::Sized,
        Self: Node<Output = (O1, O2)>,
    {
        Connector {
            node1: self, 
            node2: Pair::new(node1, node2)
        }
    }
}

// IntoNode System
pub trait ExplicitNode<I, O> {
    /// Process a batch of data
    fn process(&mut self, input: I) -> O;
    /// Reset signal propogates through pipeline
    fn reset(&mut self) {}
    /// Get number of examples left, defaults to same as previous remaining data
    fn data_remaining(&self, before: usize) -> usize {before}
}

impl <I, O, F> ExplicitNode<I, O> for F
where F: Fn(I) -> O {
    fn process(&mut self, input: I) -> O {
        (self)(input)
    }
}

pub struct NodeContainer<I, O, N: ExplicitNode<I, O>> {
    node: N,
    _phantom: PhantomData<(I, O)>
}

impl <I, O, N: ExplicitNode<I, O>> From<N> for NodeContainer<I, O, N> {
    fn from(node: N) -> Self {
        NodeContainer { node, _phantom: Default::default() }
    }
}

impl <I, O, N: ExplicitNode<I, O>> Node for NodeContainer<I, O, N> {
    type Input = I;
    type Output = O;

    fn process(&mut self, input: Self::Input) -> Self::Output {
        self.node.process(input)
    }

    fn reset(&mut self) {
        self.node.reset();
    }

    fn data_remaining(&self, before: usize) -> usize {
        self.node.data_remaining(before)
    }
}