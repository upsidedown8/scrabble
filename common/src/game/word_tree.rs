//! Module containing a word tree

use super::tile::Letter;

#[derive(Clone, Copy, Debug)]
pub struct NodeIndex(usize);

#[derive(Default, Debug)]
pub struct Node {
    is_terminal: bool,
    children: [Option<NodeIndex>; 26],
}
impl Node {
    pub fn set_child(&mut self, letter: Letter, idx: NodeIndex) {
        self.children[usize::from(letter)] = Some(idx);
    }
    pub fn get_child(&self, letter: Letter) -> Option<NodeIndex> {
        self.children[usize::from(letter)]
    }
    pub fn children(&self) -> impl Iterator<Item = &Option<NodeIndex>> {
        self.children.iter()
    }
    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }
    pub fn set_terminal(&mut self, is_terminal: bool) {
        self.is_terminal = is_terminal;
    }
    pub fn new(is_terminal: bool) -> Self {
        Self {
            is_terminal,
            children: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct WordTree {
    root: usize,
    nodes: Vec<Node>,
}

impl Default for WordTree {
    fn default() -> Self {
        Self {
            root: 0,
            nodes: vec![Node::new(false)],
        }
    }
}
impl WordTree {
    pub fn root_idx(&self) -> NodeIndex {
        NodeIndex(self.root)
    }

    pub fn node(&self, idx: NodeIndex) -> &Node {
        &self.nodes[idx.0]
    }
    pub fn node_mut(&mut self, idx: NodeIndex) -> &mut Node {
        &mut self.nodes[idx.0]
    }

    pub fn trace_word(&self, root: NodeIndex, word: &str) -> Option<NodeIndex> {
        let mut curr_idx = root;

        // follow a path along the tree defined by the word.
        for letter in word.chars().filter_map(Letter::new) {
            curr_idx = self.node(curr_idx).get_child(letter)?;
        }

        Some(curr_idx)
    }
    pub fn insert(&mut self, word: &str) {
        let mut curr_idx = self.root_idx();

        for letter in word.chars().filter_map(Letter::new) {
            let idx = match self.node(curr_idx).get_child(letter) {
                Some(idx) => idx,
                None => {
                    self.nodes.push(Node::default());
                    NodeIndex(self.nodes.len() - 1)
                }
            };

            // add node at `idx` as a child of the node at`curr_idx`
            self.node_mut(curr_idx).set_child(letter, idx);

            // update current node
            curr_idx = idx;
        }

        self.node_mut(curr_idx).set_terminal(true);
    }
    pub fn contains(&self, word: &str) -> bool {
        match self.trace_word(self.root_idx(), word) {
            Some(idx) => self.node(idx).is_terminal(),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_contains() {
        let mut word_tree = WordTree::default();

        word_tree.insert("abade");
        word_tree.insert("abide");
        word_tree.insert("a");
        word_tree.insert("collection");
        word_tree.insert("collect");

        assert!(!word_tree.contains("death"));
        assert!(!word_tree.contains("collecti"));
        assert!(word_tree.contains("collection"));
        assert!(word_tree.contains("abide"));
        assert!(word_tree.contains("abade"));
        assert!(!word_tree.contains("abadf"));
    }
}
