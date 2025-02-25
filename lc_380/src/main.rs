use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    data: Vec<i32>,
    index: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            data: Vec::new(),
            index: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.index.contains_key(&val) {
            false
        } else {
            self.data.push(val);
            self.index.insert(val, self.data.len() - 1);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.index.contains_key(&val) {
            false
        } else {
            let idx = self.index.remove(&val).unwrap();
            let last = self.data.pop().unwrap();
            if idx != self.data.len() {
                self.data[idx] = last;
                self.index.insert(last, idx);
            }
            true
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.data.len());
        self.data[idx]
    }
}

fn main() {
    let mut set = RandomizedSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.remove(2);
    println!("{}", set.get_random());
}
