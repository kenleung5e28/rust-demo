use std::cmp::Ordering;

#[allow(dead_code)]

pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn inorder_traversal<'a>(&'a self, acc: &mut Vec<&'a T>) {
        if let Some(left) = self.left.as_ref() {
            left.as_ref().inorder_traversal(acc);
        }
        acc.push(&self.value);
        if let Some(right) = self.right.as_ref() {
            right.as_ref().inorder_traversal(acc);
        }
    }
}

impl<T: Ord> Node<T> {
    pub fn inorder_insert(&mut self, value: T) {
        if self.value <= value {
            match self.left.as_mut() {
                None => {
                    self.left = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }))
                }
                Some(left) => {
                    left.as_mut().inorder_insert(value);
                }
            }
        } else {
            match self.right.as_mut() {
                None => {
                    self.right = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }))
                }
                Some(right) => {
                    right.as_mut().inorder_insert(value);
                }
            }
        }
    }

    pub fn inorder_search(&self, value: &T) -> bool {
        match self.value.cmp(value) {
            Ordering::Equal => true,
            Ordering::Less => self
                .left
                .as_ref()
                .map_or(false, |n| n.inorder_search(value)),
            Ordering::Greater => self
                .right
                .as_ref()
                .map_or(false, |n| n.inorder_search(value)),
        }
    }
}

pub struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Node<T>>,
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        BinarySearchTree::new()
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, value: T) {
        match self.root.as_mut() {
            None => self.root = Some(Node::new(value)),
            Some(node) => node.inorder_insert(value),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.root
            .as_ref()
            .map_or(false, |n| n.inorder_search(value))
    }

    pub fn sorted_vec(&self) -> Vec<&T> {
        let mut result = vec![];
        if let Some(ref node) = self.root {
            node.inorder_traversal(&mut result);
        }
        result
    }
}

fn main() {
    let v = vec![34, -12, 0, 13, 55, 6, 9, -12, 9, 24];
    let mut bst = BinarySearchTree::<i32>::new();
    for n in v {
        bst.insert(n);
    }
    println!("{:?}", bst.sorted_vec());
}
