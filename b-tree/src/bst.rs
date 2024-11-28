#![allow(dead_code, unused_variables, unused_mut, unused_assignments, unused_imports)]
//!
//! an implementation for BST
//!

use std::fmt;

pub struct Node<T>
where
    T: Ord,
{
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: fmt::Display + Ord> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_node(self, f)
    }
}

fn write_node<T: fmt::Display + Ord>(node: &Node<T>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(left) = &node.left {
        let _ = f.write_str(format!("{} --> {}[{}L]\n", node.val, left.val, left.val).as_str());
        let _ = write_node(left, f);
    }

    if let Some(right) = &node.right {
        let _ = f.write_str(format!("{} --> {}[{}R]\n", node.val, right.val, right.val).as_str());
        let _ = write_node(right, f);
    }

    Ok(())
}

/// a more rust-way implementation for BST
pub mod rs_bst {
    use std::cmp::Ordering;
    use std::fmt;

    pub enum RsNode<T: Ord>
    {
        Node {
            val: T,
            left: Box<RsNode<T>>,
            right: Box<RsNode<T>>,
        },
        Empty,
    }

    impl<T: Ord> RsNode<T> {
        pub fn new() -> RsNode<T> {
            RsNode::Empty
        }

        pub fn create(val: T) -> RsNode<T> {
            RsNode::Node {
                val,
                left: Box::new(RsNode::Empty),
                right: Box::new(RsNode::Empty),
            }
        }

        pub fn add(&mut self, new_value: T) {
            match self {
                RsNode::Node {
                    ref val,
                    ref mut left,
                    ref mut right,
                } => match new_value.cmp(val) {
                    Ordering::Less => left.add(new_value),
                    Ordering::Greater => right.add(new_value),
                    Ordering::Equal => return,
                },
                RsNode::Empty => {
                    *self = RsNode::create(new_value);
                }
            }
        }

        pub fn find(&self, find_value: T) -> bool {
            match self {
                RsNode::Node {
                    ref val,
                    ref left,
                    ref right,
                } => match find_value.cmp(val) {
                    Ordering::Less => left.find(find_value),
                    Ordering::Greater => right.find(find_value),
                    Ordering::Equal => true,
                },
                RsNode::Empty => false,
            }
        }
    }

    impl<T: fmt::Display + Ord> fmt::Display for RsNode<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write_node(self, f)
        }
    }

    fn write_node<T: fmt::Display + Ord>(node: &RsNode<T>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match node {
            RsNode::Node {
                ref val,
                ref left,
                ref right
            } => {
                // 访问 left 和 right 中的 val
                if let RsNode::Node { val: left_val, .. } = &**left {
                    let _ = f.write_str(format!("{} --> {}[{}L]\n", val, left_val, left_val).as_str());
                }
                let _ = write_node(left, f);

                if let RsNode::Node { val: right_val, .. } = &**right {
                    let _ = f.write_str(format!("{} --> {}[{}R]\n", val, right_val, right_val).as_str());
                }
                let _ = write_node(right, f);
            }
            RsNode::Empty => {}
        }

        Ok(())
    }
}

/// a traditional implementation for BST
pub mod tradition {
    use std::cmp::Ordering;
    use crate::bst::Node;

    impl<T> Node<T>
    where
        T: Ord,
    {
        pub fn new(root: T) -> Node<T> {
            Node {
                val: root,
                left: None,
                right: None,
            }
        }

        pub fn add(mut root: Node<T>, val: T) -> Node<T> {
            assert!(root.val != val);
            if root.val < val {
                root.right = Self::add_child(root.right, val)
            } else {
                root.left = Self::add_child(root.left, val);
            }
            root
        }

        fn add_child(child: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
            match child {
                Some(node) => Some(Box::new(Self::add(*node, val))),
                None => Some(Box::new(Node::new(val))),
            }
        }

        pub fn find(&self, val: T) -> bool {
            match &self {
                &Self {
                    val: value,
                    ..
                } => {
                    match value.cmp(&val) {
                        Ordering::Less => Node::find_with_option(&self.right, val),
                        Ordering::Equal => true,
                        Ordering::Greater => Node::find_with_option(&self.left, val)
                    }
                }
            }
        }

        pub fn find_with_option(node : &Option<Box<Node<T>>>, val: T) -> bool {
            match node {
                None => false,
                Some(node) => node.find(val),
            }
        }
    }
}

/// another traditional implementation with add(*self) for BST
pub mod se1f {
    use crate::bst::Node;

    impl<T> Node<T>
    where
        T: Ord,
    {
        // `self` must be `mut` rather than `&mut` or `&` due to :
        //  1.1 the struct must be mutable because we are going to modify it;
        //  1.2 when we modify a node, we take the ownership because it maybe changes.
        pub fn add_self(&mut self, val: T) {
            assert!(self.val != val);
            if self.val < val {
                match &mut self.right {
                    None => self.right = Some(Box::new(Node::new(val))),
                    Some(right) => right.add_self(val),
                }
            } else {
                match &mut self.left {
                    None => self.left = Some(Box::new(Node::new(val))),
                    Some(left) => left.add_self(val),
                }
            }
        }

        // fn add_self_child(child: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
        //     match child {
        //         Some(node) => Some(Box::new(node.add_self(val))),
        //         None => Some(Box::new(Node::new(val))),
        //     }
        // }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::bst::Node;
    use crate::bst::rs_bst::RsNode;

    #[test]
    fn test_rust_style(){
        let mut empty = RsNode::new();
        assert!(!empty.find(0));
        assert!(!empty.find(1));
        assert!(!empty.find(2));

        empty.add(0);
        assert!(empty.find(0));
        assert!(!empty.find(1));
        assert!(!empty.find(2));
        empty.add(2);
        assert!(empty.find(0));
        assert!(!empty.find(1));
        assert!(empty.find(2));
        empty.add(1);
        assert!(empty.find(0));
        assert!(empty.find(1));
        assert!(empty.find(2));
    }

    #[test]
    fn test_traditional_style() {
        let mut node = Node::new(0);
        assert!(node.find(0));

        node.add_self(2);
        assert!(node.find(0));
        assert!(!node.find(1));
        assert!(node.find(2));

        node.add_self(1);
        assert!(node.find(0));
        assert!(node.find(1));
        assert!(node.find(2));

    }
}