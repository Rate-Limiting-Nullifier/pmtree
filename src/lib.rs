//! # pmtree
//! Persistent Merkle Tree in Rust
//!
//! ## How it stored
//! { [0, 0, ..., 0] : depth }
//! { [0, 0, ..., 1] : next_index}
//! { Position (tuple, converted to DBKey) : Value}

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
