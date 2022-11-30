use crate::*;

/// Trait that must be implemented for Hash Function
pub trait Hasher {
    /// Native type for the hash-function
    type Fr: Copy + Eq + Default;

    /// Serializes Self::Fr
    fn serialize(value: Self::Fr) -> Value;

    /// Deserializes Self::Fr
    fn deserialize(value: Value) -> Self::Fr;

    /// Outputs the default leaf (Fr::default())
    fn default_leaf() -> Self::Fr;

    /// Calculates hash-function
    fn hash(input: &[Self::Fr]) -> Self::Fr;
}
