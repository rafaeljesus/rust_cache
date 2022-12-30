use std::{
    cmp::Eq,
    collections::{HashMap, VecDeque},
    hash::Hash,
};

/// The FIFO cache evicts the items in the order they were added
/// without any regard to how often or how many times they were accessed before
pub struct Fifo<K, V> {
    entry_map: HashMap<K, V>,
    queue: VecDeque<K>,
}

impl<K, V> Fifo<K, V>
where
    K: Eq + Hash + Copy,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            entry_map: HashMap::with_capacity(capacity),
            queue: VecDeque::with_capacity(capacity),
        }
    }

    // Time: O(1) | Space: O(n)
    pub fn set(&mut self, key: K, value: V) -> bool {
        if self.entry_map.capacity() == self.entry_map.len() {
            let front_key = match self.queue.pop_front() {
                Some(front_key) => front_key,
                None => return false,
            };
            match self.entry_map.remove_entry(&front_key) {
                Some((_, _)) => (),
                // maybe it should panic if entry not present?
                None => return false,
            }
        }
        match self.entry_map.insert(key, value) {
            Some(_) => (),
            None => self.queue.push_back(key),
        }
        true
    }

    // Time: O(1) | Space: O(1)
    pub fn get(&self, key: K) -> Option<&V> {
        match self.entry_map.get_key_value(&key) {
            Some((_, value)) => Some(value),
            None => None,
        }
    }
}
