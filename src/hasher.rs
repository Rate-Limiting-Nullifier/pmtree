/// Trait that must be implemented for Hash Function
pub trait Hasher {
    type Fr: Copy + Eq + Default;

    fn new() -> Self;
    fn hash(input: &[Self::Fr]) -> Self::Fr;
}

// TODO: import actual Poseidon and implement Hasher for it
pub struct Poseidon;

impl Hasher for Poseidon {
    type Fr = i32;

    fn new() -> Self {
        Self
    }

    fn hash(_input: &[i32]) -> i32 {
        0
    }
}
