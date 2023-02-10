use crate::*;

use std::collections::HashMap;

/// Trait that must be implemented for a Database
pub trait Database {
    /// Config for database. Default is necessary for a default() pmtree function
    type Config: Default;

    /// Creates new instance of db
    fn new(config: Self::Config) -> Result<Self>
    where
        Self: Sized;

    /// Loades existing db (existence check required)
    fn load(config: Self::Config) -> Result<Self>
    where
        Self: Sized;

    /// Returns value from db by the key
    fn get(&self, key: DBKey) -> Result<Option<Value>>;

    /// Puts the value to the db by the key
    fn put(&mut self, key: DBKey, value: Value) -> Result<()>;

    /// Batc
    fn put_batch(&mut self, subtree: HashMap<DBKey, Value>) -> Result<()>;
}
