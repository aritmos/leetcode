// Trie (Prefix Tree)
use std::collections::HashSet;

#[derive(Default)]
pub struct Trie {
    data: HashSet<String>,
}
// this isnt the actual Trie implementation that they expect
// as such this code actually runs super slow! (beats 5%)

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        Trie {
            data: HashSet::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        self.data.insert(word);
    }

    pub fn search(&self, word: String) -> bool {
        self.data.contains(&word)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        for k in self.data.iter() {
            if k.starts_with(&prefix) {
                return true;
            }
        }
        false
    }
}
