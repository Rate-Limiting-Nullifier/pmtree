use crate::*;

pub struct MerkleTree<D = DefaultDatabase, H = DefaultHasher>
where
    D: Database,
    H: Hasher,
{
    db: D,
    h: H,
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new("VectorDB")
    }
}

impl<D, H> MerkleTree<D, H>
where
    D: Database,
    H: Hasher,
{
    pub fn new(dbpath: &str) -> Self {
        Self {
            db: Database::new(dbpath),
            h: Hasher::new(),
        }
    }
}
