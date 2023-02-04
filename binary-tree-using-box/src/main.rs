#[allow(dead_code)]

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn inorder_traversal<T>(node: &Node<T>, f: fn(&T) -> ()) {
    if let Some(left) = node.left.as_ref() {
        inorder_traversal(left.as_ref(), f);
    }
    f(&node.value);
    if let Some(right) = node.right.as_ref() {
        inorder_traversal(right.as_ref(), f);
    }
}

fn main() {
    let mut root = Node {
        value: 34u32,
        left: None,
        right: Some(Box::new(Node {
            value: 66u32,
            left: None,
            right: None,
        })),
    };
    let b = root.right.as_mut().unwrap().as_mut();
    let a = Node {
        value: b.value + 200,
        left: None,
        right: None,
    };
    println!("{} {}", b.value, root.value);
    b.value = 1001;
    b.left = Some(Box::new(a));
    inorder_traversal(&root, |v| {
        println!("{v}");
    });
}
