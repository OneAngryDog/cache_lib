//!
//! eviction.rs
//!
//! Defines eviction policies for cache management.
//!

use std::collections::HashMap;
use std::hash::Hash;

pub trait EvictionPolicy<K> {
    /// Called when a new key is inserted into the cache
    ///
    /// # Parameters
    /// * `key`: The key that was inserted
    fn on_insert(&mut self, key: &K);

    /// Called when a key is accessed
    ///
    /// # Parameters
    /// * `key`: The key that was accessed
    fn on_access(&mut self, key: &K);

    /// Called when a key is removed.
    ///
    /// # Parameters
    /// * `key`: The key that was removed.
    fn on_remove(&mut self, key: &K);

    /// Determines which key should be evicted from the cache.
    ///
    /// # Returns
    /// An `Option` containing the key to evict if a suitable candidate is found, or `None`.
    fn evict(&mut self) -> Option<K>;
}

// ==============================================================================================
//                                      LRU Eviction Policy
// ==============================================================================================

pub struct LRU<K>
where
    K: Eq + Hash + Clone + Copy,
{
    use_order: HashMap<K, usize>,
    current_time: usize,
}

impl<K> LRU<K>
where
    K: Eq + Hash + Clone + Copy,
{
    /// Creates a new LRU eviction policy instance.
    ///
    /// # Returns
    /// An `LRU` instance.
    pub fn new() -> Self {
        LRU {
            use_order: HashMap::new(),
            current_time: 0,
        }
    }
}

impl<K> EvictionPolicy<K> for LRU<K>
where
    K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        self.current_time += 1;
        self.use_order.insert(key.clone(), self.current_time);
    }

    fn on_access(&mut self, key: &K) {
        self.current_time += 1;
        self.use_order.insert(key.clone(), self.current_time);
    }

    fn on_remove(&mut self, key: &K) {
        self.use_order.remove(key);
    }

    fn evict(&mut self) -> Option<K> {
        if let Some((&key, _)) = self.use_order.iter().min_by_key(|entry| entry.1) {
            self.use_order.remove(&key);
            Some(key)
        } else {
            None
        }
    }
}