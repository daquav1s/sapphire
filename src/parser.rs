use crate::lexer::Type;
use crate::tree::Tree;

pub fn parse<T>(tokens: &Vec<T>) {
    // Temporary code to test that the Tree works.
    let mut tree: Tree = Tree::default();
    let root = tree.node(Type::Keyword("root".to_string()));

    let child1 = tree.add_child(root, Type::Identifier("child1".to_string()));
    let child2 = tree.add_child(root, Type::Identifier("child2".to_string()));

    tree.add_child(child1, Type::Int(1));
    tree.add_child(child1, Type::Int(2));

    tree.add_child(child2, Type::String("child2.1".to_string()));

    tree.print(root);
}
