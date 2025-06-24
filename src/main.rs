use std::{
    fmt::{self},
    usize,
};

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_end_of_word: false,
        }
    }
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for char_code in word.to_lowercase().chars() {
            // Ignore non-alphabetic characters
            if !char_code.is_ascii_alphabetic() {
                continue;
            }

            let index = (char_code as usize) - ('a' as usize);
            let next_node = &mut current_node.children[index];
            if next_node.is_none() {
                *next_node = Some(Box::new(TrieNode::new()));
            }
            current_node = next_node.as_mut().unwrap();
        }
        current_node.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for char_code in word.to_lowercase().chars() {
            // Ignore non-alphabetic characters only a-z
            if !char_code.is_ascii_alphabetic() {
                return false;
            }
            let index = (char_code as usize) - ('a' as usize);
            match &current_node.children[index] {
                Some(node) => current_node = node,
                None => return false, // Path doesn't exist, word not found
            }
        }
        // Return true only if it's marked as the end of a word
        current_node.is_end_of_word
    }
}

// Implement Debug for TrieNode
impl fmt::Debug for TrieNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_map = f.debug_map();
        for (i, child_opt) in self.children.iter().enumerate() {
            if let Some(_child) = child_opt {
                // We don't want to recursively print the whole tree here
                let char_val = (b'a' + i as u8) as char;
                debug_map.entry(&char_val, &"Some(TrieNode)"); // Indicate child exists
            }
        }
        debug_map.finish()?;
        write!(f, ", is_end_of_word: {}", self.is_end_of_word)
    }
}

fn main() {
    let mut dictionary_trie = Trie::new();

    // Populate with some English words
    dictionary_trie.insert("apple");
    dictionary_trie.insert("ape'");
    dictionary_trie.insert("ball");
    println!("{:?}", dictionary_trie.root);
}

// test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(trie.contains("hello"));
        assert!(!trie.contains("hell"));
        trie.insert("hell");
        assert!(trie.contains("hell"));
    }

    #[test]
    fn test_case_insensitivity() {
        let mut trie = Trie::new();
        trie.insert("Hello");
        assert!(trie.contains("hello"));
        assert!(trie.contains("HELLO"));
    }

    #[test]
    fn test_non_alphabetic_characters() {
        let mut trie = Trie::new();
        trie.insert("apple!");
        assert!(trie.contains("apple"));
        assert!(!trie.contains("apple%"));
    }

}
