#![allow(dead_code, unused_variables, unused_mut, unused_assignments, unused_imports)]
//!
//! an implementation for Red-Black Tree
//!
//! https://en.wikipedia.org/wiki/Red%E2%80%93black_tree
//!
//! *red-black* tree is a data structure based on *[2-3-4 tree](https://en.wikipedia.org/wiki/2%E2%80%933%E2%80%934_tree)*.
//!
//! In addition to the requirements imposed on a binary search tree the following must be satisfied by a red–black tree:
//! 1. Every node is either red or black.
//! 2. All NIL nodes (figure 1) are considered black.
//! 3. A red node does not have a red child.
//! 4. Every path from a given node to any of its descendant NIL nodes goes through the same number of **black nodes**.
//! 5. (Conclusion) If a node N has exactly one child, the child must be red(and the node N itself must be black, because <3>), because if it were black, its NIL descendants
//! would sit at a different black depth than N's NIL child, violating requirement 4.
//!

use std::cmp::Ordering;
use std::fmt;
use std::mem::swap;
use std::ops::DerefMut;
use std::ptr::replace;
use crate::rbt::Rbt::Leaf;

#[derive(Debug, PartialEq, Clone)]
enum Rbt<T: Ord + fmt::Display + fmt::Debug> {
    Node {
        is_red: bool, // represent the color
        val: T,
        left: Box<Rbt<T>>,
        right: Box<Rbt<T>>,
    },
    Leaf,
}

impl<T: Ord + fmt::Display + fmt::Debug> Rbt<T> {
    pub fn new() -> Rbt<T> {
        Leaf
    }

    pub fn insert(&mut self, new_val: T) {
        self.internal_insert(new_val);
        self.update_colors(false);
    }

    pub fn search(&mut self, data: T) -> bool {
        match self {
            Rbt::Node {
                ref val,
                left,
                right,
                ..
            } => {
                match data.cmp(val) {
                    Ordering::Equal => true,
                    Ordering::Less => left.search(data),
                    Ordering::Greater => right.search(data),
                }
            }
            Leaf => false
        }
    }

    pub fn is_red(&self) -> bool {
        match self {
            Rbt::Node {
                is_red,
                ..
            } => *is_red,
            Leaf => false
        }
    }

    fn internal_insert(&mut self, new_val: T) {
        use std::mem::swap as node_swap;

        match self {
            Rbt::Node {
                is_red,
                ref val,
                ref mut left,
                ref mut right
            } => {
                let cmp_value = new_val.cmp(val);
                if cmp_value == Ordering::Less {
                    left.internal_insert(new_val);
                } else if cmp_value == Ordering::Greater {
                    right.internal_insert(new_val);
                } else {
                    return;
                }

                if self.right().is_red() && !self.left().is_red() {
                    let mut tmp = Leaf;
                    node_swap(&mut tmp, self);
                    tmp = Self::rotate(tmp, true);
                    node_swap(&mut tmp, self);
                }

                if self.left().is_red() && self.left().child(true).is_red() {
                    let mut tmp = Leaf;
                    node_swap(&mut tmp, self);
                    tmp = Self::rotate(tmp, false);
                    node_swap(&mut tmp, self);
                }

                if self.left().is_red() && self.right().is_red() {
                    self.update_colors(true);
                    self.left().update_colors(false);
                    self.right().update_colors(false);
                }
            }
            Leaf => {
                *self = Self::new_node(new_val)
            }
        }
    }

    fn rotate(mut root: Rbt<T>, left: bool) -> Rbt<T> {
        use std::mem::swap as node_swap;

        let root_color = root.is_red();

        let mut tmp = Leaf;

        // assume that we are rotating left
        // now tmp is : root.right.left
        node_swap(&mut tmp, root.child(!left).child(left));
        // now tmp is : root.right
        node_swap(&mut tmp, root.child(!left));

        tmp.update_colors(root_color);
        root.update_colors(true);

        node_swap(tmp.child(left), &mut Box::new(root));

        tmp
    }

    fn new_node(data: T) -> Rbt<T> {
        Rbt::Node {
            // every new node is red
            is_red: true,
            val: data,
            left: Box::new(Leaf),
            right: Box::new(Leaf),
        }
    }

    fn is_nil(&self) -> bool {
        match self {
            Rbt::Node { .. } => false,
            Leaf => true
        }
    }

    fn value(&mut self) -> &T {
        match self {
            Rbt::Node {
                ref val,
                ..
            } => val,
            Leaf => panic!("Attempted to get value of leaf"),
        }
    }

    fn left(&mut self) -> &mut Rbt<T> {
        self.child(true)
    }

    fn right(&mut self) -> &mut Rbt<T> {
        self.child(false)
    }

    fn child(&mut self, left: bool) -> &mut Rbt<T> {
        match self {
            Rbt::Node {
                left: child_left,
                right: child_right,
                ..
            } => {
                if left {
                    child_left.deref_mut()
                } else {
                    child_right.deref_mut()
                }
            }
            Leaf => panic!("Attempted to get child of leaf"),
        }
    }

    fn update_colors(&mut self, new_is_red: bool) {
        match self {
            Rbt::Node {
                is_red,
                ..
            } => *is_red = new_is_red,
            Leaf => {}
        }
    }

    fn output(&self) {
        match &self {
            Rbt::Node { is_red, val, left, right } => {
                println!("is_red = {is_red}, val = {val},\n\tleft = {:?},\n\tright = {:?}", left, right)
            }
            Leaf => println!("leaf")
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut tree = Rbt::new();

        tree.insert(4);
        tree.insert(2);
        tree.insert(6);
        tree.insert(1);
        tree.insert(3);
        tree.insert(5);

        assert_eq!(tree.search(4), true);
        assert_eq!(tree.search(2), true);
        assert_eq!(tree.search(6), true);
        assert_eq!(tree.search(1), true);
        assert_eq!(tree.search(3), true);
        assert_eq!(tree.search(5), true);
        assert_eq!(tree.search(0), false);
        assert_eq!(tree.search(7), false);
    }

    #[test]
    fn test_left_rotate() {
        //      1(b)
        //     / \
        //    0   3(r)
        //       / \
        //      2   4
        let mut right_side_tree = Rbt::Node {
            is_red: false,
            val: 1,
            left: Box::new(Rbt::Node {
                is_red: false,
                val: 0,
                left: Box::new(Rbt::Leaf),
                right: Box::new(Rbt::Leaf),
            }),
            right: Box::new(Rbt::Node {
                is_red: true,
                val: 3,
                left: Box::new(Rbt::Node {
                    is_red: false,
                    val: 2,
                    left: Box::new(Rbt::Leaf),
                    right: Box::new(Rbt::Leaf),
                }),
                right: Box::new(Rbt::Node {
                    is_red: false,
                    val: 4,
                    left: Box::new(Rbt::Leaf),
                    right: Box::new(Rbt::Leaf),
                }),
            }),
        };

        //      3(b)
        //     /   \
        //    1(r)  4
        //   / \
        //  0   2
        let mut left_side_tree = Rbt::Node {
            is_red: false,
            val: 3,
            left: Box::new(Rbt::Node {
                is_red: true,
                val: 1,
                left: Box::new(Rbt::Node {
                    is_red: false,
                    val: 0,
                    left: Box::new(Rbt::Leaf),
                    right: Box::new(Rbt::Leaf),
                }),
                right: Box::new(Rbt::Node {
                    is_red: false,
                    val: 2,
                    left: Box::new(Rbt::Leaf),
                    right: Box::new(Rbt::Leaf),
                }),
            }),
            right: Box::new(Rbt::Node {
                is_red: false,
                val: 4,
                left: Box::new(Rbt::Leaf),
                right: Box::new(Rbt::Leaf),
            }),
        };

        let new_tree = Rbt::rotate(right_side_tree.clone(), true);
        assert_eq!(new_tree, left_side_tree);

        let new_tree = Rbt::rotate(left_side_tree.clone(), false);
        assert_eq!(new_tree, right_side_tree);
    }

    #[test]
    fn test_insert_condition_01() {
        let mut root = Rbt::new();
        root.insert(2);
        root.insert(1);
        let target = Rbt::Node {
            is_red: false,
            val: 2,
            left: Box::new(Rbt::Node {
                is_red: true,
                val: 1,
                left: Box::new(Leaf),
                right: Box::new(Leaf),
            }),
            right: Box::new(Leaf),
        };
        assert_eq!(root, target);
    }

    #[test]
    fn test_insert_condition_02() {
        let mut root = Rbt::new();
        root.insert(2);
        root.insert(3);
        let target = Rbt::Node {
            is_red: false,
            val: 3,
            left: Box::new(Rbt::Node {
                is_red: true,
                val: 2,
                left: Box::new(Leaf),
                right: Box::new(Leaf),
            }),
            right: Box::new(Leaf),
        };
        assert_eq!(root, target);
    }

    #[test]
    fn test_insert_condition_03() {
        let mut root = Rbt::new();
        root.insert(1);
        root.insert(3);
        root.insert(5);
        root.insert(4);
        root.insert(2);

        let target = Rbt::Node {
            is_red: false,
            val: 3,
            left: Box::new(Rbt::Node {
                is_red: false,
                val: 2,
                left: Box::new(Rbt::Node {
                    is_red: true,
                    val: 1,
                    left: Box::new(Rbt::Leaf),
                    right: Box::new(Rbt::Leaf),
                }),
                right: Box::new(Rbt::Leaf),
            }),
            right: Box::new(Rbt::Node {
                is_red: false,
                val: 5,
                left: Box::new(Rbt::Node {
                    is_red: true,
                    val: 4,
                    left: Box::new(Rbt::Leaf),
                    right: Box::new(Rbt::Leaf),
                }),
                right: Box::new(Rbt::Leaf),
            }),
        };
        assert_eq!(root, target);
    }

    #[test]
    fn test_rotate_double_left_red() {
        let mut root = Rbt::new();
        root.insert(6);
        root.insert(2);
        root.insert(1);
        root.insert(0);
        root.insert(3);
        root.insert(9);
        root.insert(5);
        root.insert(7);
        root.insert(8);
        root.insert(4);
        root.insert(-1);

        for i in 0..9 {
            assert!(root.search(i));
        }

        assert_eq!(6, *root.value());

        assert_eq!(2, *root.left().value());
        assert_eq!(8, *root.right().value());

        assert_eq!(0, *root.left().left().value());
        assert_eq!(4, *root.left().right().value());

        assert_eq!(-1, *root.left().left().left().value());
        assert_eq!(1, *root.left().left().right().value());

        assert_eq!(3, *root.left().right().left().value());
        assert_eq!(5, *root.left().right().right().value());

        assert_eq!(7, *root.right().left().value());
        assert_eq!(9, *root.right().right().value());
    }

}
