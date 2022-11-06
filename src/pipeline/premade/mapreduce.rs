use std::{collections::BTreeMap, marker::PhantomData};

use crate::pipeline::Node;

/// Implements the MapReduce operation as seen here: https://research.google/pubs/pub62/
pub struct MapReduce<I, K, V, O, Map: Fn(I) -> Vec<(K, V)>, Reduce: Fn((K, Vec<V>)) -> Vec<O>> {
    map: Map,
    reduce: Reduce,
    _phantom: PhantomData<(I, K, V, O)>,
}

impl<I, K: Ord, V, O, Map: Fn(I) -> Vec<(K, V)>, Reduce: Fn((K, Vec<V>)) -> Vec<O>>
    MapReduce<I, K, V, O, Map, Reduce>
{
    pub fn new(map: Map, reduce: Reduce) -> Self {
        Self {
            map,
            reduce,
            _phantom: PhantomData::default(),
        }
    }
}

impl<I, K: Ord, V, O, Map: Fn(I) -> Vec<(K, V)>, Reduce: Fn((K, Vec<V>)) -> Vec<O>> Node
    for MapReduce<I, K, V, O, Map, Reduce>
{
    type Input = I;

    type Output = O;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        group(input.into_iter().flat_map(&self.map))
            .into_iter()
            .flat_map(&self.reduce)
            .collect()
    }
}

fn group<A, B, I>(v: I) -> BTreeMap<A, Vec<B>>
where
    A: Ord,
    I: IntoIterator<Item = (A, B)>,
{
    let mut result = BTreeMap::<A, Vec<B>>::new();
    for (a, b) in v {
        result.entry(a).or_default().push(b);
    }
    result
}
