use hex_literal::hex;
use pmtree::*;
use std::collections::HashMap;
use tiny_keccak::{Hasher as _, Keccak};

pub struct MemoryDB(HashMap<DBKey, Value>);
pub struct MyKeccak(Keccak);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MyFr([u8; 32]);

impl Database for MemoryDB {
    fn new(_dbpath: &str) -> Self {
        MemoryDB(HashMap::new())
    }

    fn load(_dbpath: &str) -> Self {
        panic!("Cannot load in-memory db!")
    }

    fn get(&self, key: DBKey) -> Option<Value> {
        self.0.get(&key).cloned()
    }

    fn put(&mut self, key: DBKey, value: Value) {
        self.0.insert(key, value);
    }

    fn delete(&mut self, key: DBKey) {
        self.0.remove(&key);
    }
}

impl From<Vec<u8>> for MyFr {
    fn from(v: Vec<u8>) -> Self {
        let v = v.try_into().unwrap();
        MyFr(v)
    }
}

impl From<MyFr> for Vec<u8> {
    fn from(v: MyFr) -> Self {
        v.0.to_vec()
    }
}

impl Hasher for MyKeccak {
    type Fr = MyFr;

    fn default_leaf() -> Self::Fr {
        MyFr([0; 32])
    }

    fn hash(input: &[Self::Fr]) -> Self::Fr {
        let mut output = [0; 32];
        let mut hasher = Keccak::v256();
        for element in input {
            hasher.update(&element.0);
        }
        hasher.finalize(&mut output);
        MyFr(output)
    }
}

#[test]
fn insert_delete() {}
