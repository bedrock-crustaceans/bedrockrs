use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
struct ClearCacheContainerValue<CachedValue> {
    internal: CachedValue,
    sequence_id: usize,
}

impl<CachedValue> ClearCacheContainerValue<CachedValue> {
    pub fn new(internal: CachedValue, sequence_id: usize) -> Self {
        Self {
            internal,
            sequence_id,
        }
    }
}

pub struct ClearCacheContainer<KeyType: Hash + Eq + Clone, CachedValue> {
    sequence_id: usize,
    pub clear_threshold: usize,
    cache_information: HashMap<KeyType, ClearCacheContainerValue<CachedValue>>,
}

#[allow(dead_code)]
impl<KeyType: Hash + Eq + Clone, CachedValue> ClearCacheContainer<KeyType, CachedValue> {
    pub fn new() -> Self {
        Self {
            sequence_id: 0,
            clear_threshold: 2048,
            cache_information: HashMap::new(),
        }
    }

    pub fn with_threshold(clear_threshold: usize) -> Self {
        Self {
            sequence_id: 0,
            clear_threshold,
            cache_information: HashMap::new(),
        }
    }

    pub fn get(&self, key: &KeyType) -> Option<&CachedValue> {
        Some(&self.cache_information.get(key)?.internal)
    }

    pub fn insert(&mut self, key: KeyType, val: CachedValue) {
        let idx = self.next_seq();
        let _ = self
            .cache_information
            .insert(key, ClearCacheContainerValue::new(val, idx)); // Drops the old value
    }

    pub fn is_cached(&self, key: &KeyType) -> bool {
        self.get(key).map_or_else(|| false, |_| true)
    }

    pub fn len(&self) -> usize {
        self.cache_information.len()
    }

    pub fn seq_id(&self) -> usize {
        self.sequence_id
    }

    fn next_seq(&mut self) -> usize {
        self.sequence_id += 1;
        self.sequence_id - 1
    }

    pub fn cull<E, F: FnMut(KeyType, CachedValue) -> Result<(), E>>(
        &mut self,
        mut on_culled: F,
    ) -> Result<(), E> {
        if self.sequence_id != 0 && self.sequence_id % self.clear_threshold != 0 {
            return Ok(());
        }
        let max_distance = self.clear_threshold / 10; // This grabs the maximum distance the seq ID can be and still be kept in the cache
        let dropped_keys = self
            .cache_information
            .iter()
            .filter_map(|(key, val)| {
                if self.sequence_id - val.sequence_id > max_distance {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        {
            for key in dropped_keys {
                let information = self.cache_information.remove(&key).unwrap();
                on_culled(key, information.internal)?;
            }
        }

        Ok(())
    }

    pub fn clear<E, F: FnMut(KeyType, CachedValue) -> Result<(), E>>(
        &mut self,
        mut on_culled: F,
    ) -> Result<(), E> {
        for (key, val) in self.cache_information.drain() {
            on_culled(key, val.internal)?;
        }
        Ok(())
    }
}
