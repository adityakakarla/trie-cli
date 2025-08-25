use crate::trie::TrieNode;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use directories::BaseDirs;

pub fn get_json_filepath() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        let base_data_dir = base_dirs.data_dir();
        let trie_filepath = base_data_dir.join(".trie-cli.json");
        if !trie_filepath.exists() {
            let trie_node = TrieNode {
                children: HashMap::new(),
                is_word: false,
            };
            write_trie(&trie_node, &trie_filepath);
        }
        return trie_filepath;
    } else {
        panic!("Could not get file path to trie file");
    }
}

fn write_trie(trie_node: &TrieNode, trie_filepath: &PathBuf) {
    let json = serde_json::to_string(&trie_node).expect("Could not convert json to string");
    let mut trie_file = File::create(&trie_filepath).expect("Could not create trie file");
    trie_file
        .write(json.as_bytes())
        .expect("Could not initialize trie file");
}

pub fn save_trie(trie_node: &TrieNode) {
    let trie_filepath = get_json_filepath();
    write_trie(trie_node, &trie_filepath);
}

pub fn read_trie() -> TrieNode {
    let trie_filepath = get_json_filepath();
    let trie_json_str = fs::read_to_string(&trie_filepath).expect("Could not read from trie file");
    let trie_node: TrieNode = serde_json::from_str(&trie_json_str).expect("Could not parse json");
    trie_node
}
