use std::marker::PhantomData;

use super::Node;

/// Connector for chaining two nodes together
pub struct Connector<N1: Node, N2: Node<Input = N1::Output>> {
    node1: N1,
    node2: N2
}

impl <N1: Node, N2: Node<Input = N1::Output>>Connector<N1, N2> {
    pub fn new(node1: N1, node2: N2) -> Self {
        Connector {node1, node2}
    }
}
/// A node that takes in T and outputs (T, T)
pub struct Duplicator<T: Clone> {
    _phantom: PhantomData<T>,
}

impl <T: Clone>Default for Duplicator<T> {
    fn default() -> Self {
        Duplicator {_phantom: PhantomData::default()}
    }
}

impl <T: Clone>Node for Duplicator<T> {
    type Input = T;
    type Output = (T, T);
    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        input.into_iter().map(|i| (i.clone(), i)).collect()
    }
}

impl <N1: Node, N2: Node<Input = N1::Output>>Node for Connector<N1, N2> {
    type Input = N1::Input;
    type Output = N2::Output;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
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
    node2: N2
}

impl <N1: Node, N2: Node>Pair<N1, N2> {
    pub fn new(node1: N1, node2: N2) -> Self {
        Pair {node1, node2}
    }
}

impl <N1: Node, N2: Node>Node for Pair<N1, N2> {
    type Input = (N1::Input, N2::Input);
    type Output = (N1::Output, N2::Output);

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        let (a, b) = input.into_iter().unzip();
        self.node1.process(a).into_iter().zip(self.node2.process(b).into_iter()).collect()
    }

    fn reset(&mut self) {
        self.node1.reset();
        self.node2.reset();
    }

    fn data_remaining(&self, before: usize) -> usize {
        usize::min(self.node1.data_remaining(before), self.node2.data_remaining(before))
    }
}