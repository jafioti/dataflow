use std::{marker::PhantomData, thread};

use crate::pipeline::*;

// Helper functions
fn add_ten(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter().map(|n| n + 10).collect()
}
fn convert_to_int(inp: Vec<String>) -> Vec<i32> {
    inp.into_iter().map(|i| i.parse::<i32>().unwrap()).collect()
}
fn greet(inp: Vec<String>) -> Vec<String> {
    inp.into_iter().map(|i| format!("Hello {}", i)).collect()
}
fn concat_strings(inp: Vec<(String, String)>) -> Vec<String> {
    inp.into_iter()
        .map(|(a, b)| format!("{}{}", a, b))
        .collect()
}

#[test]
fn test_single_pipeline() {
    let mut pipeline = add_ten.map(|i: i32| i.to_string()).chain(greet);

    let inputs = vec![12, 3443, 123, 98543];
    assert_eq!(
        Node::process(&mut pipeline, inputs),
        vec![
            "Hello 22".to_string(),
            "Hello 3453".to_string(),
            "Hello 133".to_string(),
            "Hello 98553".to_string()
        ]
    )
}

#[test]
fn test_pair_pipeline() {
    let pipeline = add_ten
        .map(|i: i32| i.to_string())
        .split(
            greet,
            convert_to_int.chain(add_ten).map(|i: i32| i.to_string()),
        )
        .chain(|(a, b): (Vec<String>, Vec<String>)| {
            a.into_iter()
                .zip(b.into_iter())
                .collect::<Vec<(String, String)>>()
        })
        .chain(concat_strings)
        .chain(greet);
    let inputs = vec![12, 3443, 123, 98543];
    let mut holder = PipelineHolder {
        pipeline: Some(pipeline),
        _phantom: Default::default(),
    };
    let outputs = run_pipeline(&mut holder, inputs);

    println!(
        "Examples left: {}",
        Node::data_remaining(&holder.pipeline.unwrap(), 0)
    );
    assert_eq!(
        outputs,
        vec![
            "Hello Hello 2232".to_string(),
            "Hello Hello 34533463".to_string(),
            "Hello Hello 133143".to_string(),
            "Hello Hello 9855398563".to_string()
        ]
    );
}

#[test]
fn test_map_reduce_pipeline() {
    let mut pipeline = MapReduce::new(
        // Count even and odd numbers
        |mut num: i32| {
            num += 10;
            vec![(num % 2 == 0, num)]
        },
        |(is_even, nums)| {
            vec![format!(
                "{}: {nums:?}",
                if is_even { "Even" } else { "Odd" }
            )]
        },
    );

    let inputs = vec![12, 3443, 124, 98543];
    assert_eq!(
        Node::process(&mut pipeline, inputs),
        vec!["Odd: [3453, 98553]", "Even: [22, 134]",]
    )
}

struct PipelineHolder<I, N: Node<I>> {
    pub pipeline: Option<N>,
    _phantom: PhantomData<I>,
}

fn run_pipeline<I, N: Node<I> + Send + 'static>(
    pipeline_holder: &mut PipelineHolder<I, N>,
    input: I,
) -> N::Output
where
    I: Send + 'static,
    N::Output: Send + 'static,
{
    let mut pipeline = std::mem::replace(&mut pipeline_holder.pipeline, None).unwrap();
    let handle = thread::spawn(move || (pipeline.process(input), pipeline));
    let (output, pipeline) = handle.join().unwrap();
    pipeline_holder.pipeline = Some(pipeline);
    output
}
