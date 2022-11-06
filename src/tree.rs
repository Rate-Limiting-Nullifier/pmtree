use crate::*;
use std::cmp::max;
use std::marker::PhantomData;

// db[DEPTH_KEY] = depth
const DEPTH_KEY: DBKey = (usize::MAX - 1).to_be_bytes();

// db[NEXT_INDEX_KEY] = next_index;
const NEXT_INDEX_KEY: DBKey = usize::MAX.to_be_bytes();

// Denotes keys (depth, index) in Merkle Tree. Can be converted to DBKey
// TODO! Think about using hashing for that
#[derive(Clone, Copy)]
struct Key(usize, usize);
impl From<Key> for DBKey {
    fn from(key: Key) -> Self {
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
    h: PhantomData<H>,
    depth: usize,
    next_index: usize,
    cache: Vec<H::Fr>,
}

impl<D, H> MerkleTree<D, H>
where
    D: Database,
    H: Hasher,
{
    /// Creates new `MerkleTree` and store it to the specified path/db
    pub fn new(depth: usize, dbpath: &str) -> Result<Self> {
        // Create new db instance
        let mut db = D::new(dbpath)?;

        // Insert depth val into db
        let depth_val = depth.to_be_bytes().to_vec();
        db.put(DEPTH_KEY, depth_val)?;

        // Insert next_index val into db
        let next_index = 0usize;
        let next_index_val = next_index.to_be_bytes().to_vec();
        db.put(NEXT_INDEX_KEY, next_index_val)?;

        // Cache nodes
        let mut cache = vec![H::default_leaf(); depth + 1];

        // Initialize one branch of the `Merkle Tree` from bottom to top
        cache[depth] = H::default_leaf();
        db.put(Key(depth, 0).into(), cache[depth].into())?;
        for i in (0..depth).rev() {
            cache[i] = H::hash(&[cache[i + 1], cache[i + 1]]);
            db.put(Key(i, 0).into(), cache[i].into())?;
        }

        Ok(Self {
            db,
            h: PhantomData,
            depth,
            next_index,
            cache,
        })
    }

    /// Loads existing Merkle Tree from the specified path/db
    pub fn load(dbpath: &str) -> Result<Self> {
        // Load existing db instance
        let db = D::load(dbpath)?;

        // Load depth & next_index values from db
        let depth = db.get(DEPTH_KEY)?.unwrap().try_into().unwrap();
        let depth = usize::from_be_bytes(depth);

        let next_index = db.get(NEXT_INDEX_KEY)?.unwrap().try_into().unwrap();
        let next_index = usize::from_be_bytes(next_index);

        // Load cache vec
        let mut cache = vec![H::default_leaf(); depth + 1];
        cache[depth] = H::default_leaf();
        for i in (0..depth).rev() {
            cache[i] = H::hash(&[cache[i + 1], cache[i + 1]]);
        }

        Ok(Self {
            db,
            h: PhantomData,
            depth,
            next_index,
            cache,
        })
    }

    /// Sets a leaf at the specified tree index
    pub fn set(&mut self, key: usize, leaf: H::Fr) -> Result<()> {
        if key >= self.capacity() {
            return Err(Error("Merkle Tree is full!".to_string()));
        }

        self.db.put(Key(self.depth, key).into(), leaf.into())?;
        self.recalculate_from(key)?;

        // Update next_index in memory
        self.next_index = max(self.next_index, key + 1);

        // Update next_index in db
        let next_index_val = self.next_index.to_be_bytes().to_vec();
        self.db.put(NEXT_INDEX_KEY, next_index_val)?;

        Ok(())
    }

    // Recalculates `Merkle Tree` from the specified key
    fn recalculate_from(&mut self, key: usize) -> Result<()> {
        let mut depth = self.depth;
        let mut i = key;

        while depth != 0 {
            let value = self.hash_couple(depth, i)?;
            i >>= 1;
            depth -= 1;
            self.db.put(Key(depth, i).into(), value.into())?;
        }

        Ok(())
    }

    // Hashes the correct couple for the key
    fn hash_couple(&self, depth: usize, key: usize) -> Result<H::Fr> {
        let b = key & !1;
        Ok(H::hash(&[
            self.get_elem(Key(depth, b))?,
            self.get_elem(Key(depth, b + 1))?,
        ]))
    }

    // Returns elem by the key
    fn get_elem(&self, key: Key) -> Result<H::Fr> {
        Ok(self
            .db
            .get(key.into())?
            .unwrap_or_else(|| self.cache[key.0].into())
            .into())
    }

    /// Deletes a leaf at the `key` by setting it to its default value
    pub fn delete(&mut self, key: usize) -> Result<()> {
        if key >= self.next_index {
            return Err(Error("The key doesn't exist!".to_string()));
        }

        self.set(key, H::default_leaf())?;

        Ok(())
    }

    /// Inserts a leaf to the next available index
    pub fn update_next(&mut self, leaf: H::Fr) -> Result<()> {
        self.set(self.next_index, leaf)?;

        Ok(())
    }

    /// Returns the root of the tree
    pub fn root(&self) -> Result<H::Fr> {
        Ok(self.db.get(Key(0, 0).into())?.unwrap().into())
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
