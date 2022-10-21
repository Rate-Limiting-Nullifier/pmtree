/// Trait that must be implemented for Hash Function
pub trait Hasher {
    type Fr: Copy + Eq + Default;

    fn new() -> Self;
    fn hash(input: &[Self::Fr]) -> Self::Fr;
}
