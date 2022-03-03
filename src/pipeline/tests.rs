use std::{num::ParseIntError, fmt::Debug, thread};

use super::{BatchStateless, Node};

// Helper functions
fn add_ten(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter().map(|n| n + 10).collect()
}
fn convert_to_string<I: ToString>(inp: I) -> String {
    inp.to_string()
}
fn convert_to_int(inp: Vec<String>) -> Vec<Result<i32, ParseIntError>> {
    inp.into_iter().map(|i| i.parse::<i32>()).collect()
}
fn greet(inp: Vec<String>) -> Vec<String> {
    inp.into_iter().map(|i| format!("Hello {}", i)).collect()
}
fn concat_strings(inp: Vec<(String, String)>) -> Vec<String> {
    inp.into_iter().map(|(a, b)| format!("{}{}", a, b)).collect()
}
fn unwrap_result<S, F: Debug>(inp: Result<S, F>) -> S {
    inp.unwrap()
}

#[test]
fn test_single_pipeline() {
    let mut pipeline = BatchStateless::new(add_ten)
        .add_fn(convert_to_string)
        .add_batch_fn(greet);

    let inputs = vec![12, 3443, 123, 98543];
    assert_eq!(pipeline.process(inputs), vec!["Hello 22".to_string(), "Hello 3453".to_string(), "Hello 133".to_string(), "Hello 98553".to_string()])
}

#[test]
fn test_pair_pipeline() {
    let pipeline = BatchStateless::new(add_ten)
        .add_fn(convert_to_string)
        .split(
            BatchStateless::new(greet), 
            BatchStateless::new(convert_to_int)
                .add_fn(unwrap_result)
                .add_node(add_ten as fn(Vec<i32>) -> Vec<i32>) // Testing the auto implementation of node on all fn pointers
                .add_fn(convert_to_string)
        ).add_node(BatchStateless::new(concat_strings))
        .add_batch_fn(greet);
    let inputs = vec![12, 3443, 123, 98543];
    let mut holder = PipelineHolder{pipeline: Some(pipeline)};
    let outputs = run_pipeline(&mut holder, inputs);

    println!("Examples left: {}", holder.pipeline.unwrap().data_remaining());
    assert_eq!(outputs, vec!["Hello Hello 2232".to_string(), "Hello Hello 34533463".to_string(), "Hello Hello 133143".to_string(), "Hello Hello 9855398563".to_string()]);
}

struct PipelineHolder<N: Node> {
    pub pipeline: Option<N>
}

fn run_pipeline<N: Node + Send + 'static>(pipeline_holder: &mut PipelineHolder<N>, input: Vec<N::Input>) -> Vec<N::Output> 
where N::Input: Send, N::Output: Send {
    let mut pipeline = std::mem::replace(&mut pipeline_holder.pipeline, None).unwrap();
    let handle = thread::spawn(move || {
        (pipeline.process(input), pipeline)
    });
    let (output, pipeline) = handle.join().unwrap();
    pipeline_holder.pipeline = Some(pipeline);
    output
}