use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct NotFoundError;

pub struct RRCache {
    map: HashMap<i32, i32>,
    values: Vec<i32>,
    size: i32,
}

// Randomly selects a candidate item and discards it to make space when necessary.
// This algorithm does not require keeping any information about the access history.
// For its simplicity, it has been used in ARM processors.
impl RRCache {
    pub fn new(size: i32) -> Self {
        Self {
            map: HashMap::with_capacity(size as usize),
            values: Vec::new(),
            size: size,
        }
    }

    // Time: O(1) | Space: O(n)
    pub fn set(&mut self, value: i32) -> bool {
        if let Some(_) = self.map.get(&value) {
            return false;
        }
        self.values.push(value);
        let last_id = self.values.len() - 1;
        match self.map.insert(value, last_id as i32) {
            Some(_) => false,
            None => true,
        }
    }

    // Time: O(1) | Space: O(1)
    pub fn get(&mut self, value: i32) -> Result<i32, NotFoundError> {
        let value = match self.map.get_key_value(&value) {
            Some((k, _)) => *k,
            None => return Err(NotFoundError),
        };
        if self.map.len() > self.size as usize {
            let mut rng = thread_rng();
            let rand_value = match self.values.choose(&mut rng) {
                Some(v) => *v,
                None => return Err(NotFoundError),
            };
            let rand_idx = match self.map.get_key_value(&rand_value) {
                Some((idx, _)) => idx,
                None => return Err(NotFoundError),
            };
            let last_idx = self.values.len() - 1;
            self.values.swap(*rand_idx as usize, last_idx);
            self.values.pop();
            self.map.remove(&rand_value);
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rr_cache() {
        let mut rr_cache = RRCache::new(10);
        assert_eq!(rr_cache.get(1), Err(NotFoundError));
        assert_eq!(rr_cache.set(1), true);
        assert_eq!(rr_cache.get(1), Ok(1));
    }
}
