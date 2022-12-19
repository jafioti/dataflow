use std::marker::PhantomData;

use super::{Node, ExplicitNode, NodeContainer};

/// Connector for chaining two nodes together
pub struct Connector<N1: Node, N2: Node<Input = N1::Output>> {
    pub node1: N1,
    pub node2: N2,
}

impl<I, T, O, E1: ExplicitNode<I, T>, E2: ExplicitNode<T, O>> Connector<NodeContainer<I, T, E1>, NodeContainer<T, O, E2>> {
    pub fn new<I1: Into<NodeContainer<I, T, E1>>, I2: Into<NodeContainer<T, O, E2>>>(node1: I1, node2: I2) -> Self {
        let cont1: NodeContainer<I, T, E1> = node1.into();
        let cont2: NodeContainer<T, O, E2> = node2.into();
        Connector { node1: cont1, node2: cont2 }
    }
}
/// A node that takes in T and outputs (T, T)
pub struct Duplicator<T: Clone> {
    _phantom: PhantomData<T>,
}

impl<T: Clone> Default for Duplicator<T> {
    fn default() -> Self {
        Duplicator {
            _phantom: PhantomData::default(),
        }
    }
}

impl<T: Clone> Node for Duplicator<T> {
    type Input = T;
    type Output = (T, T);
    fn process(&mut self, input: Self::Input) -> Self::Output {
        (input.clone(), input)
    }
}

impl<N1: Node, N2: Node<Input = N1::Output>> Node for Connector<N1, N2> {
    type Input = N1::Input;
    type Output = N2::Output;

    fn process(&mut self, input: Self::Input) -> Self::Output {
        self.node2.process(self.node1.process(input))
    }

    fn data_remaining(&self, before: usize) -> usize {
        self.node2.data_remaining(self.node1.data_remaining(before))
    }

    fn reset(&mut self) {
        self.node1.reset();
        self.node2.reset();
    }
}

/// Pair contains two nodes that run in parallel (TODO: actually make parallel)
pub struct Pair<N1: Node, N2: Node> {
    node1: N1,
    node2: N2,
}

impl<N1: Node, N2: Node> Pair<N1, N2> {
    pub fn new(node1: N1, node2: N2) -> Self {
        Pair { node1, node2 }
    }
}

impl<N1: Node, N2: Node> Node for Pair<N1, N2> {
    type Input = (N1::Input, N2::Input);
    type Output = (N1::Output, N2::Output);

    fn process(&mut self, (a, b): Self::Input) -> Self::Output {
        (self.node1.process(a), self.node2.process(b))
    }

    fn reset(&mut self) {
        self.node1.reset();
        self.node2.reset();
    }

    fn data_remaining(&self, before: usize) -> usize {
        usize::min(self.node1.data_remaining(before), self.node2.data_remaining(before))
    }
}
