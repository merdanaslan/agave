use criterion::{criterion_group, criterion_main, Criterion};
use dashmap::DashMap;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use solana_sdk::pubkey::Pubkey;  // You'll need this dependency

fn bench_maps(c: &mut Criterion) {
    // Setup: Create some test Pubkeys
    let keys: Vec<Pubkey> = (0..1000).map(|_| Pubkey::new_unique()).collect();
    
    // Test RwLock<HashMap>
    c.bench_function("rwlock_hashmap_insert", |b| {
        b.iter(|| {
            let map = Arc::new(RwLock::new(HashMap::new()));
            for (i, key) in keys.iter().enumerate() {
                map.write().unwrap().insert(*key, i as u64);
            }
        })
    });

    // Test DashMap
    c.bench_function("dashmap_insert", |b| {
        b.iter(|| {
            let map = Arc::new(DashMap::new());
            for (i, key) in keys.iter().enumerate() {
                map.insert(*key, i as u64);
            }
        })
    });

    // Read benchmarks
    let rwlock_map = Arc::new(RwLock::new(HashMap::new()));
    let dash_map = Arc::new(DashMap::new());
    
    // Fill maps first
    for (i, key) in keys.iter().enumerate() {
        rwlock_map.write().unwrap().insert(*key, i as u64);
        dash_map.insert(*key, i as u64);
    }

    // Test RwLock<HashMap> reads
    c.bench_function("rwlock_hashmap_read", |b| {
        b.iter(|| {
            for key in keys.iter() {
                let _ = rwlock_map.read().unwrap().get(key);
            }
        })
    });

    // Test DashMap reads
    c.bench_function("dashmap_read", |b| {
        b.iter(|| {
            for key in keys.iter() {
                let _ = dash_map.get(key);
            }
        })
    });
}

criterion_group!(benches, bench_maps);
criterion_main!(benches);