//!
//! cache.rs
//!
//! Defines the main Cache struct and provides primary caching functionality.
//!

use std::hash::Hash;
use crate::eviction::EvictionPolicy;
use crate::store::Store;

/// The Cache struct, providing the primary caching functionality.
///
/// # Type Parameters
/// * `K`: The type of the keys in the cache. Must implement `Eq`, `Hash`, and `Clone`.
/// * `V`: The type of the values in the cache.
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    store: Store<K, V>,
    eviction_policy: Box<dyn EvictionPolicy<K>>,
    capacity: usize,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Creates a new Cache instance with the given eviction policy and capacity.
    ///
    /// # Parameters
    /// * `eviction_policy`: A boxed instance of a type implementing the `EvictionPolicy` trait.
    /// * `capacity`: The maximum number of items the cache can hold before evicting items.
    ///
    /// # Returns
    /// A `Cache` instance.
    pub fn new(eviction_policy: Box<dyn EvictionPolicy<K>>, capacity: usize) -> Self {
        Cache {
            store: Store::new(),
            eviction_policy,
            capacity,
        }
    }

    /// Inserts a key-value pair into the store.
    /// If the key already exists, it updates the value.
    ///
    /// # Parameters
    /// * `key`: The key to be inserted or updated.
    /// * `value`: The value associated with the key.
    pub fn set(&mut self, key: K, value: V) {
        if self.store.entries.len() >= self.capacity {
            if let Some(evicted_key) = self.eviction_policy.evict() {
                self.store.remove(&evicted_key);
            }
        }
        self.store.insert(key.clone(), value);
        self.eviction_policy.on_insert(&key);
    }

    /// Retrieves a value associated with a given key from the cache.
    ///
    /// # Parameters
    /// * `key`: The key associated with the value to be returned.
    ///
    /// # Returns
    /// An `Option` containing the value, or `None` if no value is found.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.store.contains_key(key) {
            self.eviction_policy.on_access(key);
            self.store.get(key)
        } else {
            None
        }
    }

    /// Removes a key-value pair from the cache.
    ///
    /// # Parameters
    /// * `key`: The key to remove.
    ///
    /// # Returns
    /// An `Option` containing the removed value if it exists, or `None` if no value is found.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.eviction_policy.on_remove(key);
        self.store.remove(key)
    }
}
