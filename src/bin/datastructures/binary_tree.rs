use std::fmt::Debug;

type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    pub left: NodeBox<T>,
    pub right: NodeBox<T>,
}

impl<T> Node<T> {
    pub fn new(_payload: T) -> Node<T> {
        Node { val: _payload, left: None, right: None }
    }

    pub fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }

    pub fn set_left(&mut self, node: Node<T>) {
        self.left = Self::boxer(node)
    }

    pub fn set_right(&mut self, node: Node<T>) {
        self.right = Self::boxer(node)
    }

    pub fn insert(&mut self, data: T)
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

    pub fn preorder_visit(&self)
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

    pub fn inorder_visit(&self)
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

    pub fn postorder_visit(&self)
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