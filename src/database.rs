use crate::*;

/// Trait that must be implemented for a Database
pub trait Database {
    fn new(dbpath: &str) -> Self;
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    fn delete(&mut self, key: &[u8]) -> Result<()>;
}
