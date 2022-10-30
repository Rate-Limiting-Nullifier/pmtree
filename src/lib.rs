//! # pmtree
//! Persistent Merkle Tree in Rust
//!
//! ## How it stored
//! { (usize::MAX - 1) : depth }
//! { (usize::MAX)     : next_index}
//! { Position (tuple - (depth, index), converted to DBKey) : Value}

pub mod database;
pub mod hasher;
pub mod tree;

pub use database::*;
pub use hasher::*;
pub use tree::MerkleTree;

/// Denotes keys in a database
pub type DBKey = [u8; 8];

/// Denotes values in a database
pub type Value = Vec<u8>;
