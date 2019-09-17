#![deny(missing_docs)]

//! Key Value Storage is a primitive, in-memory database with set, get and remove methods.

use std::collections::HashMap;

/// The struct that stores values.
/// Its "values" field is a HashMap of String keys and values.
pub struct KvStore {
    /// Where our in-memory database is stored.
    values: HashMap<String, String>,
}

impl KvStore {
    /// Create an empty instance of the database.
    /// ```
    /// # use kvs::KvStore;
    /// let mut db = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            values: HashMap::new(),
        }
    }
    /// The getter function for values.
    /// ```
    /// # use kvs::KvStore;
    /// # let mut db = KvStore::new();
    /// db.get("key".to_string());
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.values.get(&key).map(|s| s.to_owned())
    }
    /// The setter function.
    /// ```
    /// # use kvs::KvStore;
    /// # let mut db = KvStore::new();
    /// db.set("key1".to_string(), "value1".to_string())
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }
    /// Remove values.
    /// ```
    /// # use kvs::KvStore;
    /// # let mut db = KvStore::new();
    /// db.remove("key1".to_string())
    /// ```
    pub fn remove(&mut self, key: String) {
        self.values.remove(&key);
    }
}
