use crate::*;

/// Trait that must be implemented for a Database
pub trait Database {
    /// Creates new instance of db
    fn new(dbpath: &str) -> Result<Self>
    where
        Self: Sized;

    /// Loades existing db (existence check required)
    fn load(dbpath: &str) -> Result<Self>
    where
        Self: Sized;

    /// Returns value from db by the key
    fn get(&self, key: DBKey) -> Result<Option<Value>>;

    /// Puts the value to the db by the key
    fn put(&mut self, key: DBKey, value: Value) -> Result<()>;
}
