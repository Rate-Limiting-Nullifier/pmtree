use crate::*;

/// Trait that must be implemented for a Database
pub trait Database {
    /// Creates new instance of db
    fn new(dbpath: &str) -> Self;

    /// Loades existing db (existence check required)
    fn load(dbpath: &str) -> Self;

    /// Returns value from db by the key
    fn get(&self, key: DBKey) -> Option<Value>;

    /// Puts the value to the db by the key
    fn put(&mut self, key: DBKey, value: Value);

    /// Deletes the key from db
    fn delete(&mut self, key: DBKey);
}
