# Dataflow

![image](https://www.sidekickai.co/static/images/other/dag.png)

[![CI Status](https://img.shields.io/github/actions/workflow/status/Sidekick-AI/dataflow/rust.yml?style=for-the-badge&logo=github-actions&logoColor=white&branch=main)](https://github.com/Sidekick-AI/dataflow/actions)
[![Current Crates.io Version](https://img.shields.io/crates/v/dataflow.svg?style=for-the-badge&logo=rust)](https://crates.io/crates/dataflow)
[![Documentation](https://img.shields.io/badge/docs-online-5023dd.svg?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/dataflow/0.1.0/dataflow/)

Dataflow is a data processing library that provides composable primatives to build flexible, fast and statically typed data pipelines. The pipeline is a directed acyclic dataflow graph, which a dataloader can run on a seperate thread to feed data-hungry applications.

## Usage
To build a pipeline, first start with a loader Node:
```rust
use dataflow::prelude::*;

fn main() {
  let pipeline = FileLoader::from_directory("my_data_directory");
}
```
The FileLoader loads the files from the directory in a random order. Next add a transformation to it with the `map()` function:
```rust
let pipeline = FileLoader::from_directory("my_data_directory")
      .map(|(_, text)| format!("Hello {}", text)) // Add hello to each file
```
`map()` takes in a Node that processes a single sample at a time. If we want to do batch processing, we can use `.chain()` which takes a Node that can process a batch at a time.

Important note: **All functions and closures are also Nodes!** This means that whenever we want to add a stateless transformation, we could just use a function. In this case, the closure takes in a single datapoint and outputs a single datapoint. 

Now we've added "Hello " to every line, let's use a tokenizer from `dataflow_nlp` in our pipeline:
```rust
// Our tokenizer
let tokenizer = WordpieceTokenizer::load();

// Our pipeline
let pipeline = FileLoader::from_directory("my_data_directory")
      .map(|(_, text)| format!("Hello {}", text)) // Add hello to each file
      .chain(tokenizer); // Tokenize the lines

```
Great! Now our data gets efficiently tokenized in batches. Right now, we will get single tokenized sentences out of the pipeline one at a time. But what if we wanted to get batches out? Let's use a Batch node:
```rust

// Our tokenizer
let tokenizer = dataflow_nlp::tokenization::WordpieceTokenizer::load();

// Our pipeline
let pipeline = FileLoader::from_directory("my_data_directory")
      .map(|(_, text)| format!("Hello {}", text)) // Add hello to each file
      .chain(tokenizer) // Tokenize the files
      .chain(Batch::new(64)); // Create batches of 64
```
That's it! We'll now get batches of 64 tokenized sentences.

### Loader Nodes
As discussed before, everything in the pipeline implements the `Node` trait. RandomLoader is also a node! So the question arises, since data originates from it, and since Nodes need an *input* and an *output*, what does it take as an input? Simple, it takes as input Vec<()>, which is what the pipeline will start with, and produces data (Vec<String>) to send through the pipeline. This pattern is the same across all Nodes where data originates.
  
### Custom Nodes
In fact, you can implement your own Nodes as well, by implementing the `Node` trait!
```rust
pub trait Node<Input> {
    type Output;

    /// Process a batch of data
    fn process(&mut self, input: Input) -> Self::Output;
    /// Reset signal propogates through pipeline
    fn reset(&mut self) {}
    /// Get number of examples left
    fn data_remaining(&self, before: usize) -> usize {
        before // Defaults to same as previous remaining data
    }
}
```
Your custom nodes can then be inserted directly into the pipeline!
  
### Dataloader
Since we built this cool pipeline, what can we do with it? Well for starters, we could simply call process() and feed in some data:
```rust
// The RandomLoader takes in a () for each sample, so we pass in a batch as Vec<()>
let output: Vec<Vec<Vec<String>>> = pipeline.process(vec![(); 128])

// Output should now contain 2 batches of 64 tokenized sentences from our files with "Hello" prepended.
```

Let's do something cooler. Let's put it in a Dataloader and use it in an ML training loop:
```rust
// Make the dataloader
let mut dataloader = Dataloader(pipeline);

// Training loop
for example in &mut dataloader {
   // Now example is a vector of tokenized strings!
   // Do with them what you please...
}
```

To Do:
- [ ] Make dataloader use a multiqueue instead of draining all examples into buffer on main thread
- [ ] Make auto-parallel pipeline Node using rayon
- [ ] Add async ability and remote sources. (blocked by stable async traits)