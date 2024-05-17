//!
//! cache_lib
//!
//! A lightweight caching library for Rust.
//!
//! Modules:
//! - cache: Provides the main cache struct and its associated methods.
//! - eviction: Defines eviction policies for cache management.
//! - store: Implements the storage layer for the cache.
//! - utils: Contains utility functions and helpers.
//!

pub mod cache;
pub mod eviction;
pub mod store;

pub use cache::Cache;
pub use eviction::{ LRU, FIFO, LFU, MRU, RandomEviction, SLRU, SFIFO, KLRU, SecondChance, ARC };
pub use store::Store;