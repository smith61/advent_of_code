
use fxhash::FxHashMap;

#[derive(Clone, Copy, Debug)]
enum FlatTrieNode {
    Empty,
    Intermediate,
    Tail
}

impl FlatTrieNode {

    pub fn is_empty(self) -> bool {
        match self {
            FlatTrieNode::Empty => true,
            _ => false
        }
    }

    pub fn is_tail(self) -> bool {
        match self {
            FlatTrieNode::Tail => true,
            _ => false
        }
    }

}

struct FlatTrie {
    max_depth: usize,
    storage: Vec<FlatTrieNode>
}

impl FlatTrie {

    pub fn new(max_depth: usize) -> Self {
        let mut size = 1;
        for _ in 0..max_depth {
            size += size * 5;
        }

        Self {
            max_depth,
            storage: vec![FlatTrieNode::Empty; size]
        }
    }

    pub fn get_child_node(&self, node_index: usize, c: u8) -> (FlatTrieNode, usize) {
        let child_index = (node_index * 5) + FlatTrie::character_to_index(c) + 1;
        (self.storage[child_index], child_index)
    }

    pub fn get_max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn insert(&mut self, value: &[u8]) {
        assert!(value.len() <= self.max_depth);

        let mut current_index = 0;
        for &c in value {
            current_index = (current_index * 5) + FlatTrie::character_to_index(c) + 1;
            if self.storage[current_index].is_empty() {
                self.storage[current_index] = FlatTrieNode::Intermediate;
            }
        }

        self.storage[current_index] = FlatTrieNode::Tail;
    }

    fn character_to_index(c: u8) -> usize {
        match c {
            b'w' => 0,
            b'u' => 1,
            b'b' => 2,
            b'r' => 3,
            b'g' => 4,
            c => panic!("Unexpected character '{}'", c as char)
        }
    }

}

fn count_possibilities<'a, const SINGLE_MATCH: bool>(pattern: &'a [u8], towels: &FlatTrie, memo: &mut FxHashMap<&'a [u8], u64>) -> u64 {
    if pattern.is_empty() {
        return 1;
    }

    if !memo.contains_key(pattern) {
        let mut count = 0;
        let mut trie_index = 0;
        for pattern_index in 0..pattern.len().min(towels.get_max_depth()) {
            let (child_node, child_index) =
                towels.get_child_node(trie_index, pattern[pattern_index]);

            if child_node.is_empty() {
                break;
            }

            if child_node.is_tail() {
                count += count_possibilities::<SINGLE_MATCH>(&pattern[(pattern_index + 1)..], towels, memo);
                if SINGLE_MATCH && count != 0 {
                    return count;
                }
            }

            trie_index = child_index;
        }

        memo.insert(pattern, count);
    }

    *memo.get(pattern).unwrap()
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let mut trie = FlatTrie::new(8);
    for towel in lines.next().unwrap().split(", ") {
        trie.insert(towel.as_bytes());
    }

    lines.next().unwrap();
    let mut count = 0;

    let mut memo = FxHashMap::default();
    for line in lines {
        if count_possibilities::<true>(line.as_bytes(), &trie, &mut memo) != 0 {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let mut trie = FlatTrie::new(8);
    for towel in lines.next().unwrap().split(", ") {
        trie.insert(towel.as_bytes());
    }

    lines.next().unwrap();
    let mut count = 0;

    let mut memo = FxHashMap::default();
    for line in lines {
        count += count_possibilities::<false>(line.as_bytes(), &trie, &mut memo);
    }

    count
}
