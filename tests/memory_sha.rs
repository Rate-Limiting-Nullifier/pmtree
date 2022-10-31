use pmtree::*;
use sha2::Sha256;
use std::collections::HashMap;

pub struct MemoryDB(HashMap<DBKey, Value>);
pub struct MySha(Sha256);

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

impl Hasher for MySha {
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
fn insert_delete() {
    let mut merkle_tree = MerkleTree::<MemoryDB, MySha>::new(3, "memory");
    assert_eq!(merkle_tree.depth(), 3);
    assert_eq!(merkle_tree.capacity(), 8);
    assert_eq!(merkle_tree.leaves_set(), 0);
    // assert_eq!(merkle_tree.root(), MyFr([10; 32]));

    merkle_tree.insert(MyFr([5; 32]));
    // assert_eq!(merkle_tree.root(), MyFr([10; 32]));
    assert_eq!(merkle_tree.leaves_set(), 1);

    merkle_tree.insert(MyFr([5; 32]));
    // assert_eq!(merkle_tree.root(), MyFr([10; 32]));
    assert_eq!(merkle_tree.leaves_set(), 2);

    merkle_tree.insert(MyFr([5; 32]));
    // assert_eq!(merkle_tree.root(), MyFr([10; 32]));
    assert_eq!(merkle_tree.leaves_set(), 3);

    merkle_tree.delete(1);
    // assert_eq!(merkle_tree.root(), MyFr([10; 32]));
    assert_eq!(merkle_tree.leaves_set(), 3);

    // #[should_panic]
    // merkle_tree.delete(3);

    merkle_tree.insert(MyFr([5; 32]));
    merkle_tree.insert(MyFr([5; 32]));
    merkle_tree.insert(MyFr([5; 32]));
    merkle_tree.insert(MyFr([5; 32]));
    merkle_tree.insert(MyFr([5; 32]));
    // assert_eq!(merkle_tree.root(), MyFr([10; 32]));
    assert_eq!(merkle_tree.leaves_set(), 8);

    // #[should_panic]
    // merkle_tree.insert(MyFr([5; 32]));
}
