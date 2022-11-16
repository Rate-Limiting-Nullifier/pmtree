use pmtree::*;

use rln::circuit::Fr;
use rln::poseidon_hash::poseidon_hash;

use std::collections::HashMap;
use std::io::Cursor;
use std::str::{self, FromStr};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

struct MemoryDB(HashMap<DBKey, Value>);

#[derive(Clone, Copy, PartialEq, Eq)]
struct PoseidonHash;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct MyFr(Fr);

impl Hasher for PoseidonHash {
    type Fr = MyFr;

    fn default_leaf() -> Self::Fr {
        MyFr(Fr::from(0))
    }

    fn hash(inputs: &[Self::Fr]) -> Self::Fr {
        MyFr(poseidon_hash(
            &inputs.iter().map(|&x| x.0).collect::<Vec<_>>(),
        ))
    }
}

impl From<Vec<u8>> for MyFr {
    fn from(v: Vec<u8>) -> Self {
        let c = Cursor::new(v);
        let f = Fr::deserialize(c).unwrap();
        MyFr(f)
    }
}

impl From<MyFr> for Vec<u8> {
    fn from(v: MyFr) -> Self {
        let mut buf = vec![];

        Fr::serialize(&v.0, &mut buf).unwrap();

        buf
    }
}

impl Database for MemoryDB {
    fn new(_dbpath: &str) -> Result<Self> {
        Ok(MemoryDB(HashMap::new()))
    }

    fn load(_dbpath: &str) -> Result<Self> {
        Err(Error("Cannot load in-memory DB".to_string()))
    }

    fn get(&self, key: DBKey) -> Result<Option<Value>> {
        Ok(self.0.get(&key).cloned())
    }

    fn put(&mut self, key: DBKey, value: Value) -> Result<()> {
        self.0.insert(key, value);

        Ok(())
    }
}

#[test]
fn poseidon_memory() -> Result<()> {
    let mut mt = MerkleTree::<MemoryDB, PoseidonHash>::new(2, "abacaba")?;

    assert_eq!(mt.capacity(), 4);
    assert_eq!(mt.depth(), 2);

    let leaves = vec![
        MyFr(Fr::from_str("12345").unwrap()),
        MyFr(Fr::from_str("67891").unwrap()),
        MyFr(Fr::from_str("23456").unwrap()),
        MyFr(Fr::from_str("78912").unwrap()),
    ];

    mt.update_next(leaves[0])?;

    // for &leaf in &leaves {
    //     mt.update_next(leaf)?;
    // }

    // for (i, leaf) in leaves.iter().enumerate() {
    //     assert!(mt.verify(leaf, &mt.proof(i)?));
    // }

    Ok(())
}
