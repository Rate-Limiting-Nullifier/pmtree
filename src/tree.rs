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

/// The Merkle Tree structure
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
    root: H::Fr,
}

/// The Merkle proof structure
#[derive(Clone, PartialEq, Eq)]
pub struct MerkleProof<H: Hasher>(pub Vec<(H::Fr, u8)>);

impl<D, H> MerkleTree<D, H>
where
    D: Database,
    H: Hasher,
{
    /// Creates tree with specified depth and default "pmtree_db" dbpath.
    pub fn default(depth: usize) -> Result<Self> {
        Self::new(depth, "pmtree_db")
    }

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
        db.put(Key(depth, 0).into(), H::serialize(cache[depth]))?;
        for i in (0..depth).rev() {
            cache[i] = H::hash(&[cache[i + 1], cache[i + 1]]);
            db.put(Key(i, 0).into(), H::serialize(cache[i]))?;
        }

        let root = cache[0];

        Ok(Self {
            db,
            h: PhantomData,
            depth,
            next_index,
            cache,
            root,
        })
    }

    /// Loads existing Merkle Tree from the specified path/db
    pub fn load(dbpath: &str) -> Result<Self> {
        // Load existing db instance
        let db = D::load(dbpath)?;

        // Load root
        let root = H::deserialize(db.get(Key(0, 0).into())?.unwrap());

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
            root,
        })
    }

    /// Sets a leaf at the specified tree index
    pub fn set(&mut self, key: usize, leaf: H::Fr) -> Result<()> {
        if key >= self.capacity() {
            return Err(Error("Merkle Tree is full!".to_string()));
        }

        self.db
            .put(Key(self.depth, key).into(), H::serialize(leaf))?;
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

        loop {
            let value = self.hash_couple(depth, i)?;
            i >>= 1;
            depth -= 1;
            self.db.put(Key(depth, i).into(), H::serialize(value))?;

            if depth == 0 {
                self.root = value;
                break;
            }
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
        let res = self
            .db
            .get(key.into())?
            .map_or(self.cache[key.0], |value| H::deserialize(value));

        Ok(res)
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

    /// Batch insertion, updates the tree in parallel.
    /// Only available as a feature
    #[cfg(feature = "batch_insert")]
    pub fn batch_insert(&mut self, leaves: &[H::Fr]) -> Result<()> {
        Ok(())
    }

    /// Computes a Merkle proof for the leaf at the specified index
    pub fn proof(&self, index: usize) -> Result<MerkleProof<H>> {
        if index >= self.capacity() {
            return Err(Error("Index exceeds set size!".to_string()));
        }

        let mut witness = Vec::with_capacity(self.depth);

        let mut i = index;
        let mut depth = self.depth;
        while depth != 0 {
            i ^= 1;
            witness.push((
                self.get_elem(Key(depth, i))?,
                (1 - (i & 1)).try_into().unwrap(),
            ));
            i >>= 1;
            depth -= 1;
        }

        Ok(MerkleProof(witness))
    }

    /// Verifies a Merkle proof with respect to the input leaf and the tree root
    pub fn verify(&self, leaf: &H::Fr, witness: &MerkleProof<H>) -> bool {
        let expected_root = witness.compute_root_from(leaf);

        self.root() == expected_root
    }

    /// Returns the root of the tree
    pub fn root(&self) -> H::Fr {
        self.root
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

impl<H: Hasher> MerkleProof<H> {
    /// Computes the Merkle root by iteratively hashing specified Merkle proof with specified leaf
    pub fn compute_root_from(&self, leaf: &H::Fr) -> H::Fr {
        let mut acc = *leaf;
        for w in self.0.iter() {
            if w.1 == 0 {
                acc = H::hash(&[acc, w.0]);
            } else {
                acc = H::hash(&[w.0, acc]);
            }
        }

        acc
    }

    /// Computes the leaf index corresponding to a Merkle proof
    pub fn leaf_index(&self) -> usize {
        self.get_path_index()
            .into_iter()
            .rev()
            .fold(0, |acc, digit| (acc << 1) + usize::from(digit))
    }

    /// Returns the path indexes forming a Merkle Proof
    pub fn get_path_index(&self) -> Vec<u8> {
        self.0.iter().map(|x| x.1).collect()
    }

    /// Returns the path elements forming a Merkle proof
    pub fn get_path_elements(&self) -> Vec<H::Fr> {
        self.0.iter().map(|x| x.0).collect()
    }

    /// Returns the length of a Merkle proof
    pub fn length(&self) -> usize {
        self.0.len()
    }
}
