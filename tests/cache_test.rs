//!
//! cache_tests.rs
//!
//! Unit tests for the caching library.
//!

use cache_lib::{Cache, LRU, Store};

/// Custom struct to test the cache with complex types.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct TestKey {
    id: u64,
}

#[derive(Debug, PartialEq, Clone)]
struct TestValue {
    data: String,
}

/// Tests the basic functionality of the Cache.
#[test]
fn test_cache_basic_operations() {
    let eviction_policy = Box::new(LRU::new());
    let mut cache = Cache::new(eviction_policy, 3);

    let key1 = TestKey { id: 1 };
    let value1 = TestValue { data: "value1".to_string() };
    let key2 = TestKey { id: 2 };
    let value2 = TestValue { data: "value2".to_string() };

    // Test insertion and retrieval
    cache.set(key1.clone(), value1.clone());
    assert_eq!(cache.get(&key1), Some(&value1));

    // Test updating a value
    cache.set(key1.clone(), value2.clone());
    assert_eq!(cache.get(&key1), Some(&value2));

    // Test inserting a second key
    cache.set(key2.clone(), value1.clone());
    assert_eq!(cache.get(&key2), Some(&value1));

    // Test removing a key
    assert_eq!(cache.remove(&key1), Some(value2));
    assert_eq!(cache.get(&key1), None);
}

/// Tests the LRU eviction policy.
#[test]
fn test_lru_eviction_policy() {
    let eviction_policy = Box::new(LRU::new());
    let mut cache = Cache::new(eviction_policy, 2);

    let key1 = TestKey { id: 1 };
    let value1 = TestValue { data: "value1".to_string() };
    let key2 = TestKey { id: 2 };
    let value2 = TestValue { data: "value2".to_string() };
    let key3 = TestKey { id: 3 };
    let value3 = TestValue { data: "value3".to_string() };

    // Insert two key-value pairs
    cache.set(key1.clone(), value1.clone());
    cache.set(key2.clone(), value2.clone());

    // Access key1 to make it recently used
    assert_eq!(cache.get(&key1), Some(&value1));

    // Insert another key-value pair to trigger eviction
    cache.set(key3.clone(), value3.clone());

    // Check that key2 has been evicted
    assert_eq!(cache.get(&key2), None);
    assert_eq!(cache.get(&key1), Some(&value1));
    assert_eq!(cache.get(&key3), Some(&value3));
}

/// Tests the Store functionality.
#[test]
fn test_store_operations() {
    let mut store = Store::new();

    let key1 = TestKey { id: 1 };
    let value1 = TestValue { data: "value1".to_string() };
    let key2 = TestKey { id: 2 };
    let value2 = TestValue { data: "value2".to_string() };

    // Test insertion and retrieval
    store.insert(key1.clone(), value1.clone());
    assert_eq!(store.get(key1), Some(&value1));

    // Test updating a value
    store.insert(key1.clone(), value2.clone());
    assert_eq!(store.get(key1), Some(&value2));

    // Test inserting a second key
    store.insert(key2.clone(), value1.clone());
    assert_eq!(store.get(key2), Some(&value1));

    // Test removing a key
    assert_eq!(store.remove(key1), Some(value2));
    assert_eq!(store.get(key1), None);

    // Test contains_key
    assert_eq!(store.contains_key(key2), true);
    assert_eq!(store.contains_key(key1), false);
}
