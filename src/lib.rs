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

/// Denotes `Error` type, for handling DB interaction errors
#[derive(Debug)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Custom `Result` type with custom `Error` type
pub type Result<T> = std::result::Result<T, Error>;
