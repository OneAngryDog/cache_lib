//!
//! eviction.rs
//!
//! Defines eviction policies for cache management.
//!

use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::collections::{ HashMap, VecDeque };
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

// ==============================================================================================
//                                     FIFO Eviction Policy
// ==============================================================================================

pub struct FIFO<K>
where
    K: Eq + Hash + Clone + Copy,
{
    queue: VecDeque<K>
}

impl<K> FIFO<K>
where
    K: Eq + Hash + Clone + Copy,
{
    /// Creates a new FIFO eviction policy instance
    ///
    /// # Returns
    /// A `FIFO` instance.
    pub fn new() -> Self {
        FIFO {
            queue: VecDeque::new(),
        }
    }
}

impl<K> EvictionPolicy<K> for FIFO<K>
where
    K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        self.queue.push_back(key.clone());
    }

    fn on_access(&mut self, _key: &K) {
        // Do nothing on access.
    }

    fn on_remove(&mut self, key: &K) {
        if let Some(pos) = self.queue.iter().position(|x| x == key) {
            self.queue.remove(pos);
        }
    }

    fn evict(&mut self) -> Option<K> {
        self.queue.pop_front()
    }
}

// ==============================================================================================
//                                      LFU Eviction Policy
// ==============================================================================================

pub struct LFU<K>
where
    K: Eq + Hash + Clone + Copy,
{
    frequency: HashMap<K, usize>
}

impl<K> LFU<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    /// Creates a new LFU eviction policy instance.
    ///
    /// # Returns
    /// An `LFU` instance.
    pub fn new() -> Self {
        LFU {
            frequency: HashMap::new()
        }
    }
}

impl<K> EvictionPolicy<K> for LFU<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        self.frequency.insert(key.clone(), 1);
    }

    fn on_access(&mut self, key: &K) {
        if let Some(count) = self.frequency.get_mut(key) {
            *count += 1
        }
    }

    fn on_remove(&mut self, key: &K) {
        self.frequency.remove(key);
    }

    fn evict(&mut self) -> Option<K> {
        if let Some((&key, _)) = self.frequency.iter().min_by_key(|entry| entry.1) {
            self.frequency.remove(&key);
            Some(key)
        } else {
            None
        }
    }
}

// ==============================================================================================
//                                      MRU Eviction Policy
// ==============================================================================================

pub struct MRU<K>
where
    K: Eq + Hash + Clone + Copy,
{
    use_order: HashMap<K, usize>,
    current_time: usize,
}

impl<K> MRU<K>
where
    K: Eq + Hash + Clone + Copy,
{
    /// Creates a new eviction policy instance
    ///
    /// # Returns
    /// An `MRU` instance
    pub fn new() -> Self {
        MRU {
            use_order: HashMap::new(),
            current_time: 0,
        }
    }
}

impl<K> EvictionPolicy<K> for MRU<K>
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
        if let Some((&key, _)) = self.use_order.iter().max_by_key(|entry| entry.1) {
            self.use_order.remove(&key);
            Some(key)
        } else {
            None
        }
    }
}

// ==============================================================================================
//                                     Random Eviction Policy
// ==============================================================================================

pub struct RandomEviction<K>
where
    K: Eq + Hash + Clone + Copy,
{
    keys: HashMap<K, ()>
}

impl<K> RandomEviction<K>
where
    K: Eq + Hash + Clone + Copy,
{
    /// Creates a new Random Eviction policy instance.
    ///
    /// # Returns
    /// A `RandomEviction` instance.
    pub fn new() -> Self {
        RandomEviction {
            keys: HashMap::new()
        }
    }
}

impl<K> EvictionPolicy<K> for RandomEviction<K>
where
    K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        self.keys.insert(key.clone(), ());
    }

    fn on_access(&mut self, _key: &K) {
        // Do nothing on access
    }

    fn on_remove(&mut self, key: &K) {
        self.keys.remove(key);
    }

    fn evict(&mut self) -> Option<K> {
        let mut rng = thread_rng();
        self.keys.keys().choose(&mut rng).cloned()
    }
}

// ==============================================================================================
//                                     SLRU Eviction Policy
// ==============================================================================================

pub struct SLRU<K>
where
    K: Eq + Hash + Clone + Copy,
{
    probationary: LRU<K>,
    protected: LRU<K>,
    probationary_capacity: usize,
    protected_capacity: usize,
}

impl<K> SLRU<K>
where
    K: Eq + Hash + Clone + Copy,
{
    /// Creates a new SLRU eviction policy instance
    ///
    /// # Parameters
    /// * `probationary_capacity`: The capacity of the probationary segment.
    /// * `protected_capacity`: The capacity of the protected segment.
    ///
    /// # Returns
    /// An `SLRU` instance
    pub fn new(probationary_capacity: usize, protected_capacity: usize) -> Self {
        SLRU {
            probationary: LRU::new(),
            protected: LRU::new(),
            probationary_capacity,
            protected_capacity,
        }
    }

    fn move_to_protected(&mut self, key: &K) {
        if self.protected.use_order.len() >= self.protected_capacity {
            if let Some(evicted_key) = self.protected.evict() {
                self.protected.on_remove(&evicted_key);
            }
        }
        self.probationary.on_remove(key);
        self.protected.on_insert(key);
    }
}

impl<K> EvictionPolicy<K> for SLRU<K>
where
    K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        if self.probationary.use_order.len() >= self.probationary_capacity {
            if let Some(evicted_key) = self.probationary.evict() {
                self.protected.on_remove(&evicted_key);
            }
        }
        self.probationary.on_insert(key);
    }

    fn on_access(&mut self, key: &K) {
        if self.probationary.use_order.contains_key(key) {
            self.move_to_protected(key);
        }
        self.protected.on_access(key);
    }

    fn on_remove(&mut self, key: &K) {
        if self.probationary.use_order.contains_key(key) {
            self.probationary.on_remove(key);
        } else {
            self.protected.on_remove(key);
        }
    }

    fn evict(&mut self) -> Option<K> {
        if self.probationary.use_order.len() >= self.probationary_capacity {
            self.probationary.evict()
        } else {
            self.protected.evict()
        }
    }
}