use hex_literal::hex;
use pmtree::*;
use std::collections::HashMap;
use tiny_keccak::Keccak;

pub struct MemoryDB(HashMap<DBKey, Value>);
pub struct MyKeccak(Keccak);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MyFr([u8; 32]);

impl Database for MemoryDB {
    fn new(dbpath: &str) -> Self {
        todo!()
    }

    fn load(dbpath: &str) -> Self {
        todo!()
    }

    fn get(&self, key: DBKey) -> Option<Value> {
        todo!()
    }

    fn put(&mut self, key: DBKey, value: Value) {
        todo!()
    }

    fn delete(&mut self, key: DBKey) {
        todo!()
    }
}

impl From<Vec<u8>> for MyFr {
    fn from(v: Vec<u8>) -> Self {
        todo!()
    }
}

impl From<MyFr> for Vec<u8> {
    fn from(v: MyFr) -> Self {
        todo!()
    }
}

impl Hasher for MyKeccak {
    type Fr = MyFr;

    fn new() -> Self {
        todo!()
    }

    fn default_leaf() -> Self::Fr {
        todo!()
    }

    fn hash(input: &[Self::Fr]) -> Self::Fr {
        todo!()
    }
}

#[test]
fn insert_delete() {}
