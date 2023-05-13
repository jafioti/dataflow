mod node;
pub use node::*;
mod premade;
pub use premade::*;
mod loader;
pub use loader::*;
mod connectors;
pub use connectors::*;

#[cfg(test)]
mod tests;

pub struct Pipeline;

impl<I> Node<I> for Pipeline {
    type Output = I;

    fn process(&mut self, input: I) -> I {
        input
    }
}
