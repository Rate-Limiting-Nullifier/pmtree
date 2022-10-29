use crate::*;

/// Trait that must be implemented for Hash Function
pub trait Hasher {
    /// Native type for the hash-function
    type Fr: Copy + Eq + Default + From<Value> + Into<Value>;

    /// Creates new hash-function instance
    fn new() -> Self;

    /// Outputs the default leaf (Fr::default())
    fn default_leaf() -> Self::Fr;

    /// Calculates hash-function
    fn hash(input: &[Self::Fr]) -> Self::Fr;
}
