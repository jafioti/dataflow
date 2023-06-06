use rand::{prelude::SliceRandom, thread_rng};
use std::{collections::VecDeque, thread};

use crate::pipeline::Node;

pub struct Dataloader<T> {
    pipeline: Option<Box<dyn Node<Vec<()>, Output = Vec<T>> + Send>>,
    last_pipeline_length: usize,
    buffer: VecDeque<T>,
    load_block_size: usize,
    buffer_size: usize,
    #[allow(clippy::type_complexity)]
    loading_process:
        Option<thread::JoinHandle<(Box<dyn Node<Vec<()>, Output = Vec<T>> + Send>, Vec<T>)>>,
}

impl<T: Send + 'static> Dataloader<T> {
    pub fn new(mut pipeline: impl Node<Vec<()>, Output = Vec<T>> + Send + 'static) -> Self {
        pipeline.reset();
        Dataloader {
            pipeline: Some(Box::new(pipeline)),
            buffer: VecDeque::new(),
            last_pipeline_length: 0,
            load_block_size: 1000,
            buffer_size: 1000,
            loading_process: None,
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
            || self.pipeline.as_ref().unwrap().data_remaining(0) == 0
        {
            return;
        }

        // Launch loading thread
        let mut pipeline = self.pipeline.take().unwrap();
        let load_block_size = self.load_block_size;
        self.loading_process = Some(thread::spawn(move || {
            let mut data = pipeline.process(vec![(); load_block_size]);
            data.shuffle(&mut thread_rng());
            (pipeline, data)
        }));
    }

    pub fn len(&mut self) -> usize {
        let pipeline_data = if let Some(p) = &self.pipeline {
            let l = p.data_remaining(0);
            self.last_pipeline_length = l;
            l
        } else {
            self.last_pipeline_length
        };
        pipeline_data + self.buffer.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    pub fn iter_len(&mut self) -> LenIterDataloader<T> {
        LenIterDataloader { dataloader: self }
    }
}

impl<T: Send + 'static> Iterator for Dataloader<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Check if the loading thread is finished
            if let Some(process) = &self.loading_process {
                if process.is_finished() || self.buffer.is_empty() {
                    // Unload thread
                    let process = self.loading_process.take().unwrap();
                    let (pipeline, data) = process.join().unwrap();
                    self.pipeline = Some(pipeline);
                    self.buffer.extend(data);
                }
            }
            // Launch thread if not currently running and buffer running low
            if self.buffer.len() < self.buffer_size && self.pipeline.is_some() {
                if self.pipeline.as_ref().unwrap().data_remaining(0) == 0 && self.buffer.is_empty()
                {
                    self.pipeline.as_mut().unwrap().reset();
                    return None;
                }
                self.load_block();
            }

            // Get data from buffer
            if let Some(d) = self.buffer.pop_front() {
                return Some(d);
            } else if let Some(process) = self.loading_process.take() {
                let (pipeline, data) = process.join().unwrap();
                self.pipeline = Some(pipeline);
                self.buffer.extend(data);
                if let Some(d) = self.buffer.pop_front() {
                    return Some(d);
                }
            }
        }
    }
}

pub struct LenIterDataloader<'a, T> {
    dataloader: &'a mut Dataloader<T>,
}

impl<'a, T: Send + 'static> Iterator for LenIterDataloader<'a, T> {
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.dataloader.next()?;
        Some((item, self.dataloader.len()))
    }
}
