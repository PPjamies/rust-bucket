use crate::binary_tree::Node;

mod binary_tree;

fn main() {
    let mut root = Node::new(10);
    root.set_left(Node::new(5));
    root.set_right(Node::new(15));

    println!("arr {:#?}\n", root);

    println!("Pre-order traversal...");
    root.preorder_visit();

    println!("In-order traversal...");
    root.inorder_visit();

    println!("Post-order traversal...");
    root.postorder_visit();
}