use std::marker::PhantomData;

use super::Node;

impl<I, T, O, A: Node<I, Output = T>, B: Node<T, Output = O>> Node<I> for (A, B) {
    type Output = O;

    fn process(&mut self, input: I) -> Self::Output {
        self.1.process(self.0.process(input))
    }

    fn data_remaining(&self, before: usize) -> usize {
        self.1.data_remaining(self.0.data_remaining(before))
    }

    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
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

impl<T: Clone> Node<T> for Duplicator<T> {
    type Output = (T, T);
    fn process(&mut self, input: T) -> Self::Output {
        (input.clone(), input)
    }
}

/// Pair contains two nodes that run in parallel (TODO: actually make parallel)
pub struct Pair<I0, I1, N1: Node<I0>, N2: Node<I1>> {
    pub node1: N1,
    pub node2: N2,
    _phantom: PhantomData<(I0, I1)>,
}

impl<I1, O1, N1: Node<I1, Output = O1>, I2, O2, N2: Node<I2, Output = O2>> Pair<I1, I2, N1, N2> {
    pub fn new(node1: N1, node2: N2) -> Self {
        Pair {
            node1,
            node2,
            _phantom: Default::default(),
        }
    }
}

impl<I1, I2, N1: Node<I1>, N2: Node<I2>> Node<(I1, I2)> for Pair<I1, I2, N1, N2> {
    type Output = (N1::Output, N2::Output);

    fn process(&mut self, (a, b): (I1, I2)) -> Self::Output {
        (self.node1.process(a), self.node2.process(b))
    }

    fn reset(&mut self) {
        self.node1.reset();
        self.node2.reset();
    }

    fn data_remaining(&self, before: usize) -> usize {
        usize::min(
            self.node1.data_remaining(before),
            self.node2.data_remaining(before),
        )
    }
}
