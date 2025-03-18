use rand::Rng;
use std::collections::HashMap;

pub struct RandomizedSet {
    map: HashMap<i32, usize>,
    vec: Vec<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }

        self.map.insert(val, self.vec.len());
        self.vec.push(val);
        true
    }

    pub fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val) {
            None => false,
            Some(i) => {
                self.vec.swap_remove(i);

                if i < self.vec.len() {
                    self.map.insert(self.vec[i], i);
                }

                true
            }
        }
    }

    pub fn get_random(&self) -> i32 {
        if self.vec.len() < 1 {
            return 0;
        }

        let index = rand::thread_rng().gen_range(0..self.vec.len());
        self.vec[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1));
    }

    #[test]
    fn test_remove() {
        let mut set = RandomizedSet::new();
        set.insert(1);
        assert!(set.remove(1));
        assert!(!set.remove(1));
    }

    #[test]
    fn test_get_random() {
        let mut set = RandomizedSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let random_value = set.get_random();
        assert!(0 < random_value && random_value < 4);
    }

    #[test]
    fn test_get_random_empty() {
        let set = RandomizedSet::new();
        assert!(set.get_random() == 0);
    }
}