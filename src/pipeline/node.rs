use super::{Duplicator, Map, Pair};

pub trait Node<Input> {
    type Output;

    /// Process a batch of data
    fn process(&mut self, input: Input) -> Self::Output;
    /// Reset signal propogates through pipeline
    fn reset(&mut self) {}
    /// Get number of examples left
    fn data_remaining(&self, before: usize) -> usize {
        before
    } // Defaults to same as previous remaining data
}

impl<I, O, F: Fn(I) -> O> Node<I> for F {
    type Output = O;
    fn process(&mut self, input: I) -> Self::Output {
        (self)(input)
    }
}

pub trait ExtendNode<Input, Output, E: Node<Input, Output = Output>> {
    fn chain<O, N: Node<Output, Output = O>>(self, node: N) -> (E, N);
}

impl<Input, Output, E: Node<Input, Output = Output>> ExtendNode<Input, Output, E> for E {
    fn chain<O, N: Node<Output, Output = O>>(self, node: N) -> (E, N)
    where
        Self: std::marker::Sized,
    {
        (self, node)
    }
}

pub trait ExtendNodeSplit<Input, Output: Clone, E: Node<Input, Output = Output>> {
    #[allow(clippy::type_complexity)]
    fn split<O1, O2, E1: Node<Output, Output = O1>, E2: Node<Output, Output = O2>>(
        self,
        node1: E1,
        node2: E2,
    ) -> (E, Duplicator<Output>, Pair<Output, Output, E1, E2>);
}

impl<Input, Output: Clone, E: Node<Input, Output = Output>> ExtendNodeSplit<Input, Output, E>
    for E
{
    #[allow(clippy::type_complexity)]
    fn split<O1, O2, E1: Node<Output, Output = O1>, E2: Node<Output, Output = O2>>(
        self,
        node1: E1,
        node2: E2,
    ) -> (E, Duplicator<Output>, Pair<Output, Output, E1, E2>) {
        (self, Duplicator::default(), Pair::new(node1, node2))
    }
}

pub trait ExtendNodePair<Input, Out1, Out2, E: Node<Input, Output = (Out1, Out2)>> {
    #[allow(clippy::type_complexity)]
    fn pair<F1, F2, N1: Node<Out1, Output = F1>, N2: Node<Out2, Output = F2>>(
        self,
        node1: N1,
        node2: N2,
    ) -> (E, Pair<Out1, Out2, N1, N2>);
}

impl<Input, Out1, Out2, E: Node<Input, Output = (Out1, Out2)>> ExtendNodePair<Input, Out1, Out2, E>
    for E
{
    fn pair<F1, F2, N1: Node<Out1, Output = F1>, N2: Node<Out2, Output = F2>>(
        self,
        node1: N1,
        node2: N2,
    ) -> (E, Pair<Out1, Out2, N1, N2>) {
        (self, Pair::new(node1, node2))
    }
}

pub trait ExtendNodeMap<Input, Output, E: Node<Input, Output = Vec<Output>>> {
    #[allow(clippy::type_complexity)]
    fn map<O, N: Node<Output, Output = O>>(self, node: N) -> (E, Map<Output, N>);
}

impl<Input, Output, E: Node<Input, Output = Vec<Output>>> ExtendNodeMap<Input, Output, E> for E {
    fn map<O, N: Node<Output, Output = O>>(self, node: N) -> (E, Map<Output, N>)
    where
        Self: std::marker::Sized,
    {
        (self, Map::new(node))
    }
}
