use crate::*;

pub struct MerkleTree<D, H>
where
    D: Database,
    H: Hasher,
{
    db: D,
    h: H,
    depth: usize,
    next_index: usize,
}

impl<D, H> MerkleTree<D, H>
where
    D: Database,
    H: Hasher,
{
    /// Creates new `MerkleTree`
    pub fn new(depth: usize, dbpath: &str) -> Self {
        Self {
            db: Database::new(dbpath),
            h: Hasher::new(),
            depth,
            next_index: 0,
        }
    }

    /// Inserts a leaf to the next available index
    pub fn insert(&mut self, leaf: H::Fr) {
        todo!()
    }

    /// Deletes a  leaf at the `key` by setting it to its default value
    pub fn delete(&mut self, key: usize) {
        todo!()
    }

    /// Returns the root of the tree
    pub fn root(&self) -> H::Fr {
        todo!()
    }

    /// Returns the total number of leaves set
    pub fn leaves_set(&self) -> usize {
        self.next_index
    }

    /// Returns the capacity of the tree, i.e. the maximum number of leaves
    pub fn capacity(&self) -> usize {
        1 << self.depth
    }

    /// Returns the depth of the tree
    pub fn depth(&self) -> usize {
        self.depth
    }
}
