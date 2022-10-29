use crate::*;

// db[DEPTH_KEY] = depth
const DEPTH_KEY: DBKey = [0; 8];

// db[NEXT_INDEX_KEY] = next_index;
const NEXT_INDEX_KEY: DBKey = [1; 8];

// Denotes keys (depth, index) in Merkle Tree. Can be converted to DBKey
struct Key(usize, usize);
impl From<Key> for DBKey {
    fn from(key: Key) -> DBKey {
        let cantor_pairing = (key.0 + key.1) * (key.0 + key.1 + 1) / 2 + key.1;
        cantor_pairing.to_be_bytes()
    }
}

/// Merkle Tree implementation
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
    /// Creates new `MerkleTree` and store it to the specified path/db
    pub fn new(depth: usize, dbpath: &str) -> Result<Self> {
        // Create new db instance
        let mut db = D::new(dbpath);

        // Insert depth val into db
        let depth_val = depth.to_be_bytes().to_vec();
        db.put(DEPTH_KEY, depth_val)?;

        // Insert next_index val into db
        let next_index = 0usize;
        let next_index_val = next_index.to_be_bytes().to_vec();
        db.put(NEXT_INDEX_KEY, next_index_val)?;

        // Initialize one branch of the `Merkle Tree` from bottom to top
        let mut prev = H::default_leaf();
        db.put(Key(depth, 0).into(), prev.into())?;
        for i in (0..depth).rev() {
            prev = H::hash(&[prev, prev]);
            db.put(Key(i, 0).into(), prev.into())?;
        }

        Ok(Self {
            db,
            h: H::new(),
            depth,
            next_index,
        })
    }

    /// Loads existing Merkle Tree from the specified path/db
    pub fn load(dbpath: &str) -> Result<Self> {
        // Load existing db instance
        let db = D::load(dbpath);

        // Load depth & next_index values from db
        // TODO: proper handling instead of unwrap
        let depth = db.get(DEPTH_KEY)?.unwrap().try_into().unwrap();
        let depth = usize::from_be_bytes(depth);

        let next_index = db.get(NEXT_INDEX_KEY)?.unwrap().try_into().unwrap();
        let next_index = usize::from_be_bytes(next_index);

        Ok(Self {
            db,
            h: H::new(),
            depth,
            next_index,
        })
    }

    /// Inserts a leaf to the next available index
    pub fn insert(&mut self, leaf: H::Fr) -> Result<()> {
        todo!()
    }

    // Recalculates `Merkle Tree` from the specified key
    fn recalculate_from(&mut self, key: usize) {
        todo!()
    }

    /// Deletes a  leaf at the `key` by setting it to its default value
    pub fn delete(&mut self, key: usize) -> Result<()> {
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
