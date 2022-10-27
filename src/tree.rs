use crate::*;

pub struct MerkleTree<D, H>
where
    D: Database,
    H: Hasher,
{
    db: D,
    h: H,
    depth: u8,
}

impl<D, H> MerkleTree<D, H>
where
    D: Database,
    H: Hasher,
{
    pub fn new(depth: u8, dbpath: &str) -> Self {
        Self {
            db: Database::new(dbpath),
            h: Hasher::new(),
            depth,
        }
    }
}
