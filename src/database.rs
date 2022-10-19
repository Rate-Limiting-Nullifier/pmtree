use crate::*;

/// Trait that must be implemented for a Database
pub trait Database {
    fn new(dbpath: &str) -> Self;
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn put(&mut self, key: &[u8]) -> Result<()>;
    fn delete(&mut self, key: &[u8]) -> Result<()>;
}

pub struct VecDB;

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
