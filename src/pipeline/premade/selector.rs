use std::marker::PhantomData;

use itertools::Itertools;

use crate::pipeline::Node;

/// Equally selects from N nodes that all take in the same input and give the same output
///
/// ### Example
/// ```
/// use dataflow::prelude::*;
///
/// // Combine two file loading pipelines together, each taking Vec<()> as input and outputting Vec<(PathBuf, Vec<u8>)>
/// let combined_pipe = BalancedSelector::default()
///     .add_node(FileLoader::new(vec!["file1".into()]))
///     .add_node(FileLoader::new(vec!["file2".into()]).map(|(path, file)| {
///         // Per-pipeline processing here
///         (path, file)
///     }));
/// ```
#[derive(Default)]
pub struct BalancedSelector<I, O> {
    nodes: Vec<Box<dyn Node<Vec<I>, Output = Vec<O>> + Send>>,
    _phantom: PhantomData<(I, O)>,
}

impl<I, O> BalancedSelector<I, O> {
    pub fn add_node<N: Node<Vec<I>, Output = Vec<O>> + 'static + Send>(mut self, node: N) -> Self {
        self.nodes.push(Box::new(node));
        self
    }
}

impl<I, O> Node<Vec<I>> for BalancedSelector<I, O> {
    type Output = Vec<O>;

    fn process(&mut self, mut input: Vec<I>) -> Self::Output {
        // Distribute the inputs amoung the nodes in proportion to their remaining data
        let remaining_data = self
            .nodes
            .iter()
            .map(|i| i.data_remaining(usize::MAX))
            .collect_vec();
        let total_remaining_data = remaining_data.iter().sum::<usize>() as f64;
        remaining_data
            .into_iter()
            .enumerate()
            .flat_map(|(index, i)| {
                let proportion = i as f64 / total_remaining_data;
                if input.is_empty() {
                    return vec![];
                }
                self.nodes[index].process(
                    input
                        .drain(..((input.len() as f64 * proportion) as usize).min(input.len()))
                        .collect(),
                )
            })
            .collect()
    }

    fn data_remaining(&self, before: usize) -> usize {
        self.nodes.iter().map(|n| n.data_remaining(before)).sum()
    }

    fn reset(&mut self) {
        for node in &mut self.nodes {
            node.reset();
        }
    }
}
