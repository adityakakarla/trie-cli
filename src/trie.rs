use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Serialize, Deserialize)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_word: bool,
}

impl TrieNode {
    pub fn add_word(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode {
                children: HashMap::new(),
                is_word: false,
            });
        }
        node.is_word = true;
    }

    pub fn search_word(&mut self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            match node.children.get_mut(&c) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        node.is_word
    }

    pub fn search_prefix(&mut self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            match node.children.get_mut(&c) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        true
    }

    pub fn match_pattern(&self, pattern: &str) -> Vec<String> {
        let mut queue = VecDeque::new();
        queue.push_back((self, String::new()));

        for c in pattern.chars() {
            for _ in 0..queue.len() {
                let (node, word) = queue.pop_front().expect("Could not pop value from queue");
                match c {
                    '.' => {
                        for (child, next_node) in node.children.iter() {
                            queue.push_back((next_node, format!("{}{}", word, child)));
                        }
                    }
                    _ => {
                        if let Some(next_node) = node.children.get(&c) {
                            queue.push_back((next_node, format!("{}{}", word, c)));
                        }
                    }
                }
            }
        }

        let result = queue
            .into_iter()
            .filter(|(node, _)| node.is_word)
            .map(|(_, word)| word)
            .collect();
        result
    }
}
