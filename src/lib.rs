//! # pmtree
//! Persistent Merkle tree in Rust

pub mod database;
pub mod hasher;
pub mod tree;

pub use database::*;
pub use hasher::*;
pub use tree::MerkleTree;

/// Type denoting errors that occur when interacting
/// with Database
#[derive(Debug)]
pub struct Error(String);

/// Custom `Result` type/alias; used for interaction
/// with Database
pub type Result<T> = std::result::Result<T, Error>;

/// Type that denotes default database for pmtree
pub type DefaultDatabase = VecDB;

/// Type that denotes default hash-function for pmtree
pub type DefaultHasher = Poseidon;
