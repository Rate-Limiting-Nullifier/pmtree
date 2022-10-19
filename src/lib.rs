pub struct Error(String);
pub type Result<T> = std::result::Result<T, Error>;

pub struct VecDB;
pub struct Poseidon;

pub type DefaultDatabase = VecDB;
pub type DefaultHasher = Poseidon;

impl Hasher for Poseidon {
    type Fr = i32;

    fn hash(_input: &[i32]) -> i32 {
        0
    }
}

impl Database for VecDB {
    fn new(dbpath: &str) -> Self {
        Self {}
    }
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(Some(vec![]))
    }
    fn put(&mut self, key: &[u8]) -> Result<()> {
        Ok(())
    }
    fn delete(&mut self, key: &[u8]) -> Result<()> {
        Ok(())
    }
}

pub trait Hasher {
    type Fr: Copy + Eq + Default;

    fn hash(input: &[Self::Fr]) -> Self::Fr;
}

pub trait Database {
    fn new(dbpath: &str) -> Self;
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn put(&mut self, key: &[u8]) -> Result<()>;
    fn delete(&mut self, key: &[u8]) -> Result<()>;
}

pub struct MerkleTree<D = DefaultDatabase, H = DefaultHasher>
where
    D: Database,
    H: Hasher,
{
    db: D,
    h: H,
}

impl MerkleTree {}

impl<D, H> MerkleTree<D, H>
where
    D: Database,
    H: Hasher,
{
}
