use std::marker::PhantomData;

use super::{Connector, Duplicator, Map, Pair};

pub trait Node {
    type Input;
    type Output;

    /// Process a batch of data
    fn process(&mut self, input: Self::Input) -> Self::Output;
    /// Reset signal propogates through pipeline
    fn reset(&mut self) {}
    /// Get number of examples left
    fn data_remaining(&self, before: usize) -> usize {
        before
    } // Defaults to same as previous remaining data
}

pub trait ExtendNode<Input, Output, E: ExplicitNode<Input, Output>> {
    fn node<O, N: ExplicitNode<Output, O>, T: Into<NodeContainer<Output, O, N>>>(
        self,
        node: T,
    ) -> Connector<NodeContainer<Input, Output, E>, NodeContainer<Output, O, N>>;
}

impl<Input, Output, E: ExplicitNode<Input, Output>, In: Into<NodeContainer<Input, Output, E>>>
    ExtendNode<Input, Output, E> for In
{
    fn node<O, N: ExplicitNode<Output, O>, T: Into<NodeContainer<Output, O, N>>>(
        self,
        node: T,
    ) -> Connector<NodeContainer<Input, Output, E>, NodeContainer<Output, O, N>>
    where
        Self: std::marker::Sized,
    {
        Connector {
            node1: self.into(),
            node2: node.into(),
        }
    }
}

pub trait ExtendNodeSplit<Input, Output: Clone, E: ExplicitNode<Input, Output>> {
    #[allow(clippy::type_complexity)]
    fn split<
        O1,
        O2,
        E1: ExplicitNode<Output, O1>,
        T1: Into<NodeContainer<Output, O1, E1>>,
        E2: ExplicitNode<Output, O2>,
        T2: Into<NodeContainer<Output, O2, E2>>,
    >(
        self,
        node1: T1,
        node2: T2,
    ) -> Connector<
        Connector<NodeContainer<Input, Output, E>, Duplicator<Output>>,
        Pair<NodeContainer<Output, O1, E1>, NodeContainer<Output, O2, E2>>,
    >;
}

impl<
        Input,
        Output: Clone,
        E: ExplicitNode<Input, Output>,
        In: Into<NodeContainer<Input, Output, E>>,
    > ExtendNodeSplit<Input, Output, E> for In
{
    #[allow(clippy::type_complexity)]
    fn split<
        O1,
        O2,
        E1: ExplicitNode<Output, O1>,
        T1: Into<NodeContainer<Output, O1, E1>>,
        E2: ExplicitNode<Output, O2>,
        T2: Into<NodeContainer<Output, O2, E2>>,
    >(
        self,
        node1: T1,
        node2: T2,
    ) -> Connector<
        Connector<NodeContainer<Input, Output, E>, Duplicator<Output>>,
        Pair<NodeContainer<Output, O1, E1>, NodeContainer<Output, O2, E2>>,
    > {
        Connector {
            node1: Connector {
                node1: self.into(),
                node2: Duplicator::default(),
            },
            node2: Pair::new(node1, node2),
        }
    }
}

pub trait ExtendNodePair<Input, Out1, Out2, E: ExplicitNode<Input, (Out1, Out2)>> {
    #[allow(clippy::type_complexity)]
    fn pair<
        F1,
        F2,
        N1: ExplicitNode<Out1, F1>,
        T1: Into<NodeContainer<Out1, F1, N1>>,
        N2: ExplicitNode<Out2, F2>,
        T2: Into<NodeContainer<Out2, F2, N2>>,
    >(
        self,
        node1: T1,
        node2: T2,
    ) -> Connector<
        NodeContainer<Input, (Out1, Out2), E>,
        Pair<NodeContainer<Out1, F1, N1>, NodeContainer<Out2, F2, N2>>,
    >;
}

impl<
        Input,
        Out1,
        Out2,
        E: ExplicitNode<Input, (Out1, Out2)>,
        In: Into<NodeContainer<Input, (Out1, Out2), E>>,
    > ExtendNodePair<Input, Out1, Out2, E> for In
{
    fn pair<
        F1,
        F2,
        N1: ExplicitNode<Out1, F1>,
        T1: Into<NodeContainer<Out1, F1, N1>>,
        N2: ExplicitNode<Out2, F2>,
        T2: Into<NodeContainer<Out2, F2, N2>>,
    >(
        self,
        node1: T1,
        node2: T2,
    ) -> Connector<
        NodeContainer<Input, (Out1, Out2), E>,
        Pair<NodeContainer<Out1, F1, N1>, NodeContainer<Out2, F2, N2>>,
    > {
        Connector {
            node1: self.into(),
            node2: Pair::new(node1, node2),
        }
    }
}

pub trait ExtendNodeMap<Input, Output, E: ExplicitNode<Input, Vec<Output>>> {
    #[allow(clippy::type_complexity)]
    fn map<O, N: ExplicitNode<Output, O>, T: Into<NodeContainer<Output, O, N>>>(
        self,
        node: T,
    ) -> Connector<
        NodeContainer<Input, Vec<Output>, E>,
        NodeContainer<Vec<Output>, Vec<O>, NodeTraitContainer<Map<Output, O, NodeContainer<Output, O, N>>>>,
    >;
}

impl<
        Input,
        Output,
        E: ExplicitNode<Input, Vec<Output>>,
        In: Into<NodeContainer<Input, Vec<Output>, E>>,
    > ExtendNodeMap<Input, Output, E> for In
{
    fn map<O, N: ExplicitNode<Output, O>, T: Into<NodeContainer<Output, O, N>>>(
        self,
        node: T,
    ) -> Connector<
        NodeContainer<Input, Vec<Output>, E>,
        NodeContainer<Vec<Output>, Vec<O>, NodeTraitContainer<Map<Output, O, NodeContainer<Output, O, N>>>>,
    >
    where
        Self: std::marker::Sized,
    {
        Connector {
            node1: self.into(),
            node2: Map::new(node).into(),
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
    fn data_remaining(&self, before: usize) -> usize {
        before
    }
}

impl<I, O, F> ExplicitNode<I, O> for F
where
    F: Fn(I) -> O,
{
    fn process(&mut self, input: I) -> O {
        (self)(input)
    }
}

pub struct NodeContainer<I, O, N: ExplicitNode<I, O>> {
    node: N,
    _phantom: PhantomData<(I, O)>,
}

impl<I, O, N: ExplicitNode<I, O>> NodeContainer<I, O, N> {
    pub fn new(node: N) -> Self {
        NodeContainer {
            node,
            _phantom: Default::default(),
        }
    }
}

impl<I, O, N: ExplicitNode<I, O>> From<N> for NodeContainer<I, O, N> {
    fn from(node: N) -> Self {
        NodeContainer {
            node,
            _phantom: Default::default(),
        }
    }
}

impl<I, O, N: ExplicitNode<I, O>> Node for NodeContainer<I, O, N> {
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

// Implementing Into<NodeContainer> for Node
pub struct NodeTraitContainer<N: Node> {
    pub node: N
}

impl <I, O, N: Node<Input=I, Output=O>> ExplicitNode<I, O> for NodeTraitContainer<N> {
    fn process(&mut self, input: I) -> O {
        self.node.process(input)
    }

    fn data_remaining(&self, before: usize) -> usize {
        self.node.data_remaining(before)
    }

    fn reset(&mut self) {
        self.node.reset()
    }
}

impl <I, O, N: Node<Input=I, Output=O>> From<N> for NodeContainer<I, O, NodeTraitContainer<N>> {
    fn from(node: N) -> Self {
        NodeContainer {
            node: NodeTraitContainer { node },
            _phantom: Default::default(),
        }
    }
}