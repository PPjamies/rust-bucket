use std::fmt::Debug;

type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    left: NodeBox<T>,
    right: NodeBox<T>,
}

impl<T> Node<T> {
    fn new(_payload: T) -> Node<T> {
        Node { val: _payload, left: None, right: None }
    }

    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }

    fn set_left(&mut self, node: Node<T>) {
        self.left = Self::boxer(node)
    }

    fn set_right(&mut self, node: Node<T>) {
        self.right = Self::boxer(node)
    }

    fn insert(&mut self, data: T)
    where
        T: PartialEq + PartialOrd,
    {
        if self.val > data {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data))
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data))
            }
        }
    }

    fn preorder_visit(&self)
    where
        T: Debug,
    {
        println!("({:?})", self.val);
        if let Some(ref left) = self.left {
            left.preorder_visit();
        }
        if let Some(ref right) = self.right {
            right.preorder_visit();
        }
    }

    fn inorder_visit(&self)
    where
        T: Debug,
    {
        if let Some(ref left) = self.left {
            left.inorder_visit();
        }
        println!("({:?})", self.val);
        if let Some(ref right) = self.right {
            right.inorder_visit();
        }
    }

    fn postorder_visit(&self)
    where
        T: Debug,
    {
        if let Some(ref left) = self.left {
            left.postorder_visit()
        }
        if let Some(ref right) = self.right {
            right.postorder_visit()
        }
        println!("({:?})", self.val);
    }
}

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