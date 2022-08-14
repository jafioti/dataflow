#![allow(non_snake_case)]
//! Mako is a high performance data handling library
/// Batching module contains several utilities for dealing with batches, such as shuffling and sorting batches
pub mod batching;
/// Dataloader module contains the main dataloader struct, as well as dataloader utilities
pub mod dataloader;
/// Pipeline module contains the dataflow pipeline struct, as well as all pipeline utilities
pub mod pipeline;
/// Tokenization module handles all tokenization and untokenization
#[cfg(not(doctest))]
pub mod tokenization;
/// Vocab module contains the mako vocab object and the functions to load different vocabularies
pub mod vocab;
