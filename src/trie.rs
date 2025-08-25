use crate::operations::{get_json_filepath, save_trie};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}
