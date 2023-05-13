#![allow(clippy::type_complexity)]

use std::marker::PhantomData;

use crate::pipeline::Node;

pub struct Map<I, N: Node<I>> {
    _phantom: PhantomData<I>,
    node: N,
}

impl<I, O, E: Node<I, Output = O>> Map<I, E> {
    pub fn new(node: E) -> Self {
        Map {
            _phantom: PhantomData::default(),
            node,
        }
    }
}

impl<I, O, N: Node<I, Output = O>> Node<Vec<I>> for Map<I, N> {
    type Output = Vec<O>;

    fn process(&mut self, input: Vec<I>) -> Self::Output {
        input.into_iter().map(|i| self.node.process(i)).collect()
    }
}

pub trait ExtendNodeMap<Input, Output, E: Node<Input, Output = Vec<Output>>> {
    fn map<O, N: Node<Output, Output = O>>(self, node: N) -> (E, Map<Output, N>);
    fn filter_map<O, N: Node<Output, Output = Option<O>>>(
        self,
        node: N,
    ) -> (E, FilterMap<Output, N>);
    fn filter<F: FnMut(&Output) -> bool>(self, function: F) -> (E, Filter<Output, F>);
}

impl<Input, Output, E: Node<Input, Output = Vec<Output>>> ExtendNodeMap<Input, Output, E> for E {
    fn map<O, N: Node<Output, Output = O>>(self, node: N) -> (E, Map<Output, N>)
    where
        Self: std::marker::Sized,
    {
        (self, Map::new(node))
    }

    fn filter_map<O, N: Node<Output, Output = Option<O>>>(
        self,
        node: N,
    ) -> (E, FilterMap<Output, N>)
    where
        Self: std::marker::Sized,
    {
        (self, FilterMap::new(node))
    }

    fn filter<F: FnMut(&Output) -> bool>(self, function: F) -> (E, Filter<Output, F>) {
        (self, Filter::new(function))
    }
}

pub struct FilterMap<I, N: Node<I>> {
    _phantom: PhantomData<I>,
    node: N,
}

impl<I, O, E: Node<I, Output = Option<O>>> FilterMap<I, E> {
    pub fn new(node: E) -> Self {
        FilterMap {
            _phantom: PhantomData::default(),
            node,
        }
    }
}

impl<I, O, N: Node<I, Output = Option<O>>> Node<Vec<I>> for FilterMap<I, N> {
    type Output = Vec<O>;

    fn process(&mut self, input: Vec<I>) -> Self::Output {
        input
            .into_iter()
            .filter_map(|i| self.node.process(i))
            .collect()
    }
}

pub struct Filter<I, F: FnMut(&I) -> bool> {
    _phantom: PhantomData<I>,
    function: F,
}

impl<I, F: FnMut(&I) -> bool> Filter<I, F> {
    pub fn new(function: F) -> Self {
        Filter {
            _phantom: PhantomData::default(),
            function,
        }
    }
}

impl<I, F: FnMut(&I) -> bool> Node<Vec<I>> for Filter<I, F> {
    type Output = Vec<I>;

    fn process(&mut self, input: Vec<I>) -> Self::Output {
        input.into_iter().filter(|i| (self.function)(i)).collect()
    }
}
