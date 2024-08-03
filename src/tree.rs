use crate::lexer::Type;

#[derive(Debug)]
pub struct Node {
    index: usize,
    value: Type,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(index: usize, val: Type) -> Self {
        Self {
            index,
            value: val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct Tree {
    arena: Vec<Node>,
}

impl Tree {
    pub fn node(&mut self, val: Type) -> usize {
        for node in &self.arena {
            if node.value == val {
                return node.index;
            }
        }
        let index = self.arena.len();
        self.arena.push(Node::new(index, val));
        index
    }

    pub fn add_child(&mut self, parent_index: usize, child_val: Type) -> usize {
        let child_index = self.node(child_val);
        if let Some(parent_node) = self.arena.get_mut(parent_index) {
            parent_node.children.push(child_index);
            if let Some(child_node) = self.arena.get_mut(child_index) {
                child_node.parent = Some(parent_index);
            }
        }
        child_index
    }

    // Temporary printing to view the tree.
    pub fn print(&self, root: usize) {
        self.print_recursive(root, 0);
    }

    fn print_recursive(&self, node_index: usize, depth: usize) {
        if let Some(node) = self.arena.get(node_index) {
            println!("{}> {:?}", " ".repeat(depth * 2), node.value);
            for &child_index in &node.children {
                self.print_recursive(child_index, depth + 1);
            }
        }
    }
}
