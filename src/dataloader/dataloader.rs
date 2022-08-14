use rand::{prelude::SliceRandom, thread_rng};
use std::{collections::VecDeque, thread};

use crate::pipeline::Node;

pub struct Dataloader<T> {
    pipeline: Option<Box<dyn Node<Input = (), Output = T> + Send>>,
    buffer: VecDeque<T>,
    load_block_size: usize,
    buffer_size: usize,
    #[allow(clippy::type_complexity)]
    loading_process:
        Option<thread::JoinHandle<(Box<dyn Node<Input = (), Output = T> + Send>, Vec<T>)>>,
    loading_process_flag: Option<thread_control::Flag>,
}

impl<T: Send + 'static> Dataloader<T> {
    pub fn new(mut pipeline: impl Node<Input = (), Output = T> + Send + 'static) -> Self {
        pipeline.reset();
        Dataloader {
            pipeline: Some(Box::new(pipeline)),
            buffer: VecDeque::new(),
            load_block_size: 1000,
            buffer_size: 1000,
            loading_process: None,
            loading_process_flag: None,
        }
    }

    pub fn load_block_size(self, load_block_size: usize) -> Self {
        Dataloader {
            load_block_size,
            ..self
        }
    }

    pub fn buffer_size(self, buffer_size: usize) -> Self {
        Dataloader {
            buffer_size,
            ..self
        }
    }

    fn load_block(&mut self) {
        if self.loading_process.is_some()
            || self.pipeline.is_none()
            || self.pipeline.as_ref().unwrap().data_remaining() == 0
        {
            return;
        }

        // Launch loading thread
        let mut pipeline = std::mem::replace(&mut self.pipeline, None).unwrap();
        let input = vec![(); usize::min(self.load_block_size, pipeline.data_remaining())];
        let (flag, control) = thread_control::make_pair();
        self.loading_process_flag = Some(flag);
        self.loading_process = Some(thread::spawn(move || {
            let mut data = pipeline.process(input);
            data.shuffle(&mut thread_rng());
            control.stop();
            (pipeline, data)
        }));
    }

    pub fn len(&mut self) -> usize {
        let pipeline_data = if let Some(p) = &self.pipeline {
            p.data_remaining()
        } else {
            // Wait for pipeline to return before returning number
            let process = std::mem::replace(&mut self.loading_process, None).unwrap();
            self.loading_process_flag = None;
            let (pipeline, data) = process.join().unwrap();
            let len = pipeline.data_remaining();
            self.pipeline = Some(pipeline);
            self.buffer.extend(data);
            len
        };
        pipeline_data + self.buffer.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }
}

impl<T: Send + 'static> Iterator for Dataloader<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if the loading thread is finished
        if let Some(flag) = &self.loading_process_flag {
            if !flag.is_alive() {
                // Unload thread
                let process = std::mem::replace(&mut self.loading_process, None).unwrap();
                self.loading_process_flag = None;
                let (pipeline, data) = process.join().unwrap();
                self.pipeline = Some(pipeline);
                self.buffer.extend(data);
            }
        }

        // Check if we need to load more
        if self.buffer.len() < self.buffer_size && self.pipeline.is_some() {
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
                self.loading_process_flag = None;
                let (pipeline, mut data) = process.join().unwrap();
                self.pipeline = Some(pipeline);
                let returning_data = data.pop().unwrap();
                self.buffer.extend(data);
                Some(returning_data)
            }
        } else {
            Some(self.buffer.pop_front().unwrap())
        }
    }
}
