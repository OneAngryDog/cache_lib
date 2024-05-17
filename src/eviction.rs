//!
//! eviction.rs
//!
//! Defines eviction policies for cache management.
//!

use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::collections::{ HashMap, hash_map::DefaultHasher, VecDeque };
use std::hash::{ Hash, Hasher };

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
        self.use_order.entry(*key).and_modify(|e| *e = self.current_time);
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
        self.queue.retain(|x| x != key);
    }

    fn evict(&mut self) -> Option<K> {
        self.queue.pop_front()
    }
}

// ==============================================================================================
//                                      LFU Eviction Policy
// ==============================================================================================

/// Least Frequently Used
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

/// Most Recently Used
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

/// Random Eviction Policy
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

/// Segmented Least Recently Used
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
        if self.probationary.use_order.contains_key(key) {
            if self.protected.use_order.len() >= self.protected_capacity {
                if let Some(evicted_key) = self.protected.evict() {
                    self.protected.on_remove(&evicted_key);
                }
            }
            self.probationary.on_remove(key);
            self.protected.on_insert(key);
        }
    }
}

impl<K> EvictionPolicy<K> for SLRU<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        if self.probationary.use_order.len() >= self.probationary_capacity {
            if let Some(evicted_key) = self.probationary.evict() {
                self.probationary.on_remove(&evicted_key);
            }
        }
        self.probationary.on_insert(key);
    }

    fn on_access(&mut self, key: &K) {
        if self.probationary.use_order.contains_key(key) {
            self.move_to_protected(key);
        } else {
            self.protected.on_access(key);
        }
    }

    fn on_remove(&mut self, key: &K) {
        if self.probationary.use_order.contains_key(key) {
            self.probationary.on_remove(key);
        } else {
            self.protected.on_remove(key);
        }
    }

    fn evict(&mut self) -> Option<K> {
        if let Some(evicted_key) = self.probationary.evict() {
            return Some(evicted_key);
        }
        self.protected.evict()
    }
}

// ==============================================================================================
//                                  SFIFO Eviction Policy
// ==============================================================================================

/// Segmented First In First Out
pub struct SFIFO<K>
where
    K: Eq + Hash + Clone + Copy,
{
    segments: Vec<VecDeque<K>>,
    segment_capacity: usize,
}

impl<K> SFIFO<K>
where
    K: Eq + Hash + Clone + Copy,
{
    /// Creates a new SFIFO eviction policy instance
    ///
    /// # Parameters
    /// * `num_seqments`: The number of segments to divide the cache into.
    /// * `segment_capacity`: The maximum number of items each segment can hold.
    ///
    /// # Returns
    /// A `SFIFO` instance.
    pub fn new(num_segments: usize, segment_capacity: usize) -> Self {
        SFIFO {
            segments: vec![VecDeque::new(); num_segments],
            segment_capacity,
        }
    }

    /// Determines the segment index for a given key
    ///
    /// # Parameters
    /// * `key`: The key to determine the segment for.
    ///
    /// # Returns
    /// The segment index
    fn segment_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.segments.len()
    }
}

impl<K> EvictionPolicy<K> for SFIFO<K>
where
    K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        let index = self.segment_index(key);
        if self.segments[index].len() >= self.segment_capacity {
            self.segments[index].pop_front();
        }
        self.segments[index].push_back(*key);
    }

    fn on_access(&mut self, _key: &K) {
        // Do nothing on access
    }

    fn on_remove(&mut self, key: &K) {
        let index = self.segment_index(key);
        self.segments[index].retain(|x| x != key);
    }

    fn evict(&mut self) -> Option<K> {
        for segment in &mut self.segments {
            if let Some(key) = segment.pop_front() {
                return Some(key);
            }
        }
        None
    }
}

// ==============================================================================================
//                                  KLRU Eviction Policy
// ==============================================================================================

/// K-Largest Recently Used
pub struct KLRU<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    use_order: VecDeque<K>,
    k: usize,
}

impl<K> KLRU<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    /// Creates a new KLRU eviction policy instance.
    ///
    /// # Parameters
    /// * `k`: The position of the key to be evicted.
    ///
    /// # Returns
    /// A `KLRU` instance.
    pub fn new(k: usize) -> Self {
        KLRU {
            use_order: VecDeque::new(),
            k,
        }
    }
}

impl<K> EvictionPolicy<K> for KLRU<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        self.use_order.push_back(*key);
    }

    fn on_access(&mut self, key: &K) {
        self.use_order.retain(|x| x != key);
        self.use_order.push_back(*key);
    }

    fn on_remove(&mut self, key: &K) {
        self.use_order.retain(|x| x != key);
    }

    fn evict(&mut self) -> Option<K> {
        if self.use_order.len() > self.k {
            let evicted_key = self.use_order[self.use_order.len() - 1 - self.k];
            self.use_order.retain(|x| x != &evicted_key);
            Some(evicted_key)
        } else {
            None
        }
    }
}

// ==============================================================================================
//                                  Second-Chance Eviction Policy
// ==============================================================================================

/// Second-Chance Eviction Policy
pub struct SecondChance<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    queue: VecDeque<(K, bool)>,
}

impl<K> SecondChance<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    /// Creates a new Second-Chance eviction policy instance
    ///
    /// # Returns
    /// A `SecondChance` instance.
    pub fn new() -> Self {
        SecondChance {
            queue: VecDeque::new(),
        }
    }
}

impl<K> EvictionPolicy<K> for SecondChance<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        self.queue.push_back((*key, false));
    }

    fn on_access(&mut self, key: &K) {
        if let Some((_, ref mut referenced)) = self.queue.iter_mut().find(|(k, _)| k == key) {
            *referenced = true;
        }
    }

    fn on_remove(&mut self, key: &K) {
        self.queue.retain(|(k, _)| k != key);
    }

    fn evict(&mut self) -> Option<K> {
        while let Some((key, referenced)) = self.queue.pop_front() {
            if referenced {
                self.queue.push_back((key, false));
            } else {
                return Some(key);
            }
        }
        None
    }
}

// ==============================================================================================
//                                  ARC Eviction Policy
// ==============================================================================================

/// Adaptive Replacement Cache
pub struct ARC<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    t1: VecDeque<K>,
    t2: VecDeque<K>,
    b1: VecDeque<K>,
    b2: VecDeque<K>,
    p: usize,
    capacity: usize,
}

impl<K> ARC<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    /// Creates a new ARC eviction policy instance
    ///
    /// # Parameters
    /// * `capacity`: The maximum number of items the cache can hold.
    ///
    /// # Returns
    /// An `ARC` instance.
    pub fn new(capacity: usize) -> Self {
        ARC {
            t1: VecDeque::new(),
            t2: VecDeque::new(),
            b1: VecDeque::new(),
            b2: VecDeque::new(),
            p: 0,
            capacity,
        }
    }

    fn replace(&mut self, key: &K) {
        if !self.t1.is_empty() && (self.t1.len() > self.p || (self.b2.contains(key) && self.t1.len() == self.p)) {
            let old = self.t1.pop_front().unwrap();
            self.b1.push_back(old);
        } else {
            let old = self.t2.pop_front().unwrap();
            self.b2.push_back(old);
        }
    }
}

impl<K> EvictionPolicy<K> for ARC<K>
    where
        K: Eq + Hash + Clone + Copy,
{
    fn on_insert(&mut self, key: &K) {
        if self.t1.contains(key) || self.t2.contains(key) {
            return;
        }

        if self.t1.len() + self.b1.len() == self.capacity {
            if self.t1.len() < self.capacity {
                self.b1.pop_front();
                self.replace(key);
            } else {
                self.t1.pop_front();
            }
        } else if self.t1.len() + self.t2.len() + self.b1.len() + self.b2.len() >= self.capacity {
            if self.t1.len() + self.t2.len() + self.b1.len() + self.b2.len() == 2 * self.capacity {
                self.b2.pop_front();
            }
            self.replace(key);
        }

        self.t1.push_back(*key);
    }

    fn on_access(&mut self, key: &K) {
        if self.t1.contains(key) {
            self.t1.retain(|x| x != key);
            self.t2.push_back(*key);
        } else if self.t2.contains(key) {
            self.t2.retain(|x| x != key);
            self.t2.push_back(*key);
        } else if self.b1.contains(key) {
            self.p = std::cmp::min(self.capacity, self.p + std::cmp::max(self.b2.len() / self.b1.len(), 1));
            self.replace(key);
            self.b1.retain(|x| x != key);
            self.t2.push_back(*key);
        } else if self.b2.contains(key) {
            self.p = std::cmp::max(0, self.p as isize - std::cmp::max(self.b1.len() / self.b2.len(), 1) as isize) as usize;
            self.replace(key);
            self.b2.retain(|x| x != key);
            self.t2.push_back(*key);
        }
    }

    fn on_remove(&mut self, key: &K) {
        self.t1.retain(|x| x != key);
        self.t2.retain(|x| x != key);
        self.b1.retain(|x| x != key);
        self.b2.retain(|x| x != key);
    }

    fn evict(&mut self) -> Option<K> {
        if self.t1.is_empty() && self.t2.is_empty() {
            None
        } else if self.t1.len() > self.p {
            self.t1.pop_front()
        } else {
            self.t2.pop_front()
        }
    }
}
