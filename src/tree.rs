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

pub(crate) struct Key(usize, usize);

impl From<Key> for Vec<u8> {
    fn from(key: Key) -> Vec<u8> {
        todo!()
    }
}

impl<D, H> MerkleTree<D, H>
where
    D: Database,
    H: Hasher,
{
    /// Creates new `MerkleTree`
    pub fn new(depth: usize, dbpath: &str) -> Result<Self> {
        // TODO: Use open instead of new, and check if the database is exists
        let mut db: D = Database::new(dbpath);

        db.put(&Vec::<u8>::from(Key(depth, 0)), &H::default_leaf().into())?;
        for i in (0..depth).rev() {
            let prev = db.get(&Vec::<u8>::from(Key(i + 1, 0)))?.unwrap();
            db.put(
                &Vec::<u8>::from(Key(i, 0)),
                &H::hash(&[prev.clone().into(), prev.into()]).into(),
            )?;
        }

        Ok(Self {
            db,
            h: Hasher::new(),
            depth,
            next_index: 0,
        })
    }

    /// Inserts a leaf to the next available index
    pub fn insert(&mut self, leaf: H::Fr) {
        todo!()
    }

    /// Recalculates `Merkle Tree` from the specified key
    fn recalculate_from(&mut self, key: usize) {
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
