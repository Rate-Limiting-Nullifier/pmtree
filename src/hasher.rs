pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

/// Trait that must be implemented for Hash Function
pub trait Hasher {
    type Fr: Copy + Eq + Default + From<Vec<u8>> + Into<Vec<u8>>;

    fn new() -> Self;
    fn default_leaf() -> Self::Fr;
    fn hash(input: &[Self::Fr]) -> Self::Fr;
}
