use std::{collections::VecDeque, thread};
use crate::pipeline::Node;

pub struct Dataloader<N: Node<Input = ()> + Send> {
    pipeline: Option<N>,
    buffer: VecDeque<N::Output>,
    batch_size: usize,
    block_size: usize,
    loading_process: Option<thread::JoinHandle<(N, Vec<N::Output>)>>,
}

impl <N: Node<Input = ()> + Send + 'static>Dataloader<N> 
where N::Output: Send {
    pub fn new(mut pipeline: N, batch_size: usize) -> Self {
        pipeline.reset();
        Dataloader {
            pipeline: Some(pipeline),
            buffer: VecDeque::new(),
            batch_size,
            block_size: 1000,
            loading_process: None
        } 
    }

    pub fn load_block_size(self, block_size: usize) -> Self {
        Dataloader{block_size, ..self}
    }

    fn load_block(&mut self) {
        if self.loading_process.is_some() || self.pipeline.is_none() || self.pipeline.as_ref().unwrap().data_remaining() == 0 {return;}

        // Launch loading thread
        let mut pipeline = std::mem::replace(&mut self.pipeline, None).unwrap();
        let input = vec![(); usize::min(self.block_size, pipeline.data_remaining())];
        self.loading_process = Some(thread::spawn(move || {
            let data = pipeline.process(input);
            (pipeline, data)
        }));
    }

    pub fn len(&mut self) -> usize {
        let pipeline_data = if let Some(p) = &self.pipeline {
            p.data_remaining()
        } else {
            // Wait for pipeline to return before returning number
            let process = std::mem::replace(&mut self.loading_process, None).unwrap();
            let (pipeline, data) = process.join().unwrap();
            let len = pipeline.data_remaining();
            self.pipeline = Some(pipeline);
            self.buffer.extend(data);
            len
        };
        println!("Pipeline data: {} Buffer data: {}", pipeline_data, self.buffer.len());
        pipeline_data + self.buffer.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }
}

impl <N: Node<Input = ()> + Send + 'static>Iterator for Dataloader<N> 
where N::Output: Send {
    type Item = Vec<N::Output>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if we need to load more
        if self.buffer.len() < self.block_size && self.pipeline.is_some() {
            if self.pipeline.as_ref().unwrap().data_remaining() == 0 && self.buffer.is_empty() {
                self.pipeline.as_mut().unwrap().reset();
                return None;
            }
            self.load_block();
        }
        // Get examples from buffer
        if self.buffer.is_empty() {
            if self.loading_process.is_none() {
                self.pipeline.as_mut().unwrap().reset();
                None
            } else {
                let process = std::mem::replace(&mut self.loading_process, None).unwrap();
                let (pipeline, mut data) = process.join().unwrap();
                self.pipeline = Some(pipeline);
                let returning_data = Some(data.drain(..self.batch_size).collect());
                self.buffer.extend(data);
                returning_data
            }
        } else {
            Some(self.buffer.drain(..self.batch_size).collect())
        }
    }
}