//!
//! store.rs
//!
//! Establishes the storage layer for the cache.
//!

use std::collections::HashMap;
use std::hash::Hash;

/// Store struct for managing the storage of cache entries.
///
/// # Type Parameters
/// * `K`: The type of the keys in the cache. Must implement `Eq` and `Hash`.
/// * `V`: The type of values in the cache
pub struct Store<K, V>
where
    K: Eq + Hash,
{
    pub entries: HashMap<K, V>
}

impl<K, V> Store<K, V>
where
    K: Eq + Hash,
{
    /// Creates a new `Store` instance.
    ///
    /// # Returns
    /// A `Store` instance.
    pub fn new() -> Self {
        Store {
            entries: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the store.
    /// If the key already exists, it updates the value.
    ///
    /// # Parameters
    /// * `key`: The key to be inserted or updated.
    /// * `value`: The value associated with the key.
    pub fn insert(&mut self, key: K, value: V) {
        self.entries.insert(key, value);
    }

    /// Retrieves a value associated with a given key from the cache.
    ///
    /// # Parameters
    /// * `key`: The key associated with the value to be returned.
    ///
    /// # Returns
    /// An `Option` containing the value, or `None` if no value is found.
    pub fn get(&mut self, key: K) -> Option<&V> {
        self.entries.get(&key)
    }

    /// Removes a key-value pair from the cache.
    ///
    /// # Parameters
    /// * `key`: The key to remove.
    ///
    /// # Returns
    /// An `Option` containing the removed value if it exists, or `None` if no value is found.
    pub fn remove(&mut self, key: K) -> Option<V> {
        self.entries.remove(&key)
    }

    /// Checks to see if the store contains a key-value pair for the given key.
    ///
    /// # Parameters
    /// * `key`: The key to check.
    ///
    /// # Returns
    /// `True` if they key-value pair exists, otherwise `False`
    pub fn contains_key(&mut self, key: K) -> bool {
        self.entries.contains_key(&key)
    }
}
