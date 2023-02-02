<p align="center">
    <img src="./pmtree.png" width="150">
</p>

<p align="center">
    <img src="https://github.com/Rate-Limiting-Nullifier/pmtree/workflows/Build-Test-Fmt/badge.svg" width="140">
</p>

<h1 align="center">pmtree</h1>

<h3 align="center">Persistent Merkle Tree (optimized & sparse & fixed-size) in Rust</h3>

## How to use
```toml
[dependencies]
pmtree = { git = "https://github.com/Rate-Limiting-Nullifier/pmtree" }
```

## Example

In-Memory DB (HashMap) + Keccak
```rust
struct MemoryDB(HashMap<DBKey, Value>);
struct MyKeccak(Keccak);

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

    fn put_batch(&mut self, subtree: HashMap<DBKey, Value>) -> Result<()> {
        self.0.extend(subtree.into_iter());

        Ok(())
    }
}

impl Hasher for MyKeccak {
    type Fr = [u8; 32];

    fn default_leaf() -> Self::Fr {
        [0; 32]
    }

    fn serialize(value: Self::Fr) -> Value {
        value.to_vec()
    }

    fn deserialize(value: Value) -> Self::Fr {
        value.try_into().unwrap()
    }

    fn hash(input: &[Self::Fr]) -> Self::Fr {
        let mut output = [0; 32];
        let mut hasher = Keccak::v256();
        for element in input {
            hasher.update(element);
        }
        hasher.finalize(&mut output);
        output
    }
}

fn main() {
    let mut mt = MerkleTree::<MemoryDB, MyKeccak>::new(2, "abacaba").unwrap();

    assert_eq!(mt.capacity(), 4);
    assert_eq!(mt.depth(), 2);

    mt.update_next(hex!(
        "c1ba1812ff680ce84c1d5b4f1087eeb08147a4d510f3496b2849df3a73f5af95"
    ))
    .unwrap();
}
```
