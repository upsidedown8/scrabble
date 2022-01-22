//! Module containing a word tree

use super::tile::Letter;

/// Newtype containing an index for a node, so that the node
/// can be retrieved in `O(1)`.
#[derive(Clone, Copy, Debug)]
pub struct NodeIndex(usize);

/// A node in the tree, representing a letter in a word.
#[derive(Default, Debug)]
pub struct Node {
    is_terminal: bool,
    children: [Option<NodeIndex>; 26],
}
impl Node {
    /// Sets a child by key (letter)
    pub fn set_child(&mut self, letter: Letter, idx: NodeIndex) {
        self.children[usize::from(letter)] = Some(idx);
    }
    /// Gets an optional child by key (letter)
    pub fn get_child(&self, letter: Letter) -> Option<NodeIndex> {
        self.children[usize::from(letter)]
    }
    /// Gets an iterator over the node's children
    pub fn children(&self) -> impl Iterator<Item = &Option<NodeIndex>> {
        self.children.iter()
    }
    /// Checks whether the node is terminal, meaning that a word ends
    /// at this point. If this is true then a letter combination ending
    /// with this node is a valid word.
    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }
    /// Sets the `is_terminal` property
    pub fn set_terminal(&mut self, is_terminal: bool) {
        self.is_terminal = is_terminal;
    }
    /// Creates a new node with no `children`, and `is_terminal` set to
    /// the provided value.
    pub fn new(is_terminal: bool) -> Self {
        Self {
            is_terminal,
            children: Default::default(),
        }
    }
}

/// A data structure designed to store words in a compact format, so that
/// words can be validated in `O(n)` where `n` is the length of the word. The
/// tree structure means that many words with a common suffix can be traversed
/// in an efficient manor.
///
/// Example: storing the words `car`, `cart` and `cat`. (Capital letters means
/// terminal nodes).
///
/// ```txt
/// (*root)--->`c`--->`a`--->`R`--->`T`
///                    |
///                    \---->`T`
/// ```
///
/// The data structure uses an arena storage method, so all nodes are stored
/// in a single dimensional vector, with each node containing the indices of its
/// children, pointing back to the `nodes` vector.
#[derive(Debug)]
pub struct WordTree {
    root: NodeIndex,
    nodes: Vec<Node>,
}

impl Default for WordTree {
    fn default() -> Self {
        Self {
            root: NodeIndex(0),
            nodes: vec![Node::new(false)],
        }
    }
}
impl WordTree {
    /// Gets the [`NodeIndex`] for the root node.
    pub fn root_idx(&self) -> NodeIndex {
        self.root
    }

    /// Borrows a [`Node`] from a [`NodeIndex`].
    pub fn node(&self, NodeIndex(idx): NodeIndex) -> &Node {
        &self.nodes[idx]
    }
    /// Mutably borrows a [`Node`] from a [`NodeIndex`].
    pub fn node_mut(&mut self, NodeIndex(idx): NodeIndex) -> &mut Node {
        &mut self.nodes[idx]
    }

    /// Traces a path of letters described by `word`, starting from the node
    /// referred to by `root`. If the path exists, the final node is returned,
    /// otherwise [`None`] is returned.
    pub fn trace_word<I>(&self, root: NodeIndex, iter: I) -> Option<NodeIndex>
    where
        I: Iterator<Item = Letter>,
    {
        let mut curr_idx = root;

        // follow a path along the tree defined by the word.
        for letter in iter {
            curr_idx = self.node(curr_idx).get_child(letter)?;
        }

        Some(curr_idx)
    }
    /// Inserts a `word` into the tree.
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
    /// Checks whether a full word is contained within the tree.
    pub fn contains(&self, word: &str) -> bool {
        self.contains_letters(word.chars().filter_map(Letter::new))
    }
    /// Checks whether a sequence of [`Letter`]s is a valid word.
    pub fn contains_letters<I>(&self, letters: I) -> bool
    where
        I: Iterator<Item = Letter>,
    {
        match self.trace_word(self.root_idx(), letters) {
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
