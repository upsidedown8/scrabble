//! Module containing a word tree

pub struct Node {
    pub is_terminal: bool,
    pub children: [Option<usize>; 26],
}

impl Node {
    pub fn get_child(&self, ch: usize) -> Option<usize> {
        self.children[ch]
    }
}

pub struct WordTree {
    init: [Option<usize>; 26],
    nodes: Vec<Node>,
    count: usize,
}

impl WordTree {
    pub fn str_to_iter(word: &str) -> impl Iterator<Item = usize> + '_ {
        word.chars().filter_map(|ch| match ch {
            'a'..='z' => Some((ch as usize) - 97),
            'A'..='Z' => Some((ch as usize) - 65),
            _ => None,
        })
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn insert(&mut self, word: &str) {
        let mut children = &self.init;
        let mut iter = Self::str_to_iter(word).peekable();

        while let Some(ch) = iter.next() {
            let idx = match children[ch] {
                Some(idx) => idx,
                None => {
                    let idx = self.nodes.len();
                    self.nodes.push(Node {
                        is_terminal: iter.peek().is_none(),
                        children: [None; 26],
                    });
                    idx
                }
            };

            children = &self.nodes[idx].children;
        }
    }
    pub fn contains(&self, word: &str) -> bool {
        let mut children = &self.init;
        let mut iter = Self::str_to_iter(word).peekable();

        // follow a path along the tree defined by the word.
        while let Some(ch) = iter.next() {
            let idx = match children[ch] {
                // if there is a child node, and we have reached the end of the
                // string, then if the node is terminal, the word is contained,
                // otherwise the word is not contained.
                Some(idx) if iter.peek().is_none() => return self.nodes[idx].is_terminal,
                // otherwise return the index of the next child node
                Some(idx) => idx,
                // if there is no child node then the item is not in the list.
                None => break,
            };

            // update the children reference
            children = &self.nodes[idx].children;
        }

        // otherwise must be false
        false
    }
}
