#![allow(dead_code, unused_variables, unused_mut, unused_assignments, unused_imports)]

use std::cmp::{max, Ordering};
use std::fmt::Debug;
use crate::avl::util::get_height;

type AvlBoxOption<T> = Option<Box<Avl<T>>>;

#[derive(Debug)]
struct Avl<T: Ord + Debug> {
    value: T,
    height: i32,
    left: Option<Box<Avl<T>>>,
    right: Option<Box<Avl<T>>>,
}

fn display_avl<T: Ord + std::fmt::Debug>(avl: &Avl<T>, level: usize, v: &str) {
    println!(
        "{:indent$}{v}{:?}",
        "",
        avl.value,
        indent = level * 4
    );

    if let Some(ref left) = avl.left {
        display_avl(left, level + 1, "L");
    }

    if let Some(ref right) = avl.right {
        display_avl(right, level + 1, "R");
    }
}

impl<T: Ord + Debug> Avl<T> {
    pub fn new(value: T) -> Box<Avl<T>> {
        Box::new(Self {
            value,
            height: 0,
            left: None,
            right: None,
        })
    }

    pub fn search(&self, value: &T) -> bool {
        match &self {
            &Self {
                value: val,
                height,
                ref left,
                ref right
            } => {
                match val.cmp(&value) {
                    Ordering::Equal => true,
                    Ordering::Less => util::search(right, value),
                    Ordering::Greater => util::search(left, value),
                }
            }
        }
    }

    pub fn insert(mut self, new_value: T) -> Avl<T> {
        if self.value < new_value {
            self.right = util::insert(self.right, new_value);
        } else {
            self.left = util::insert(self.left, new_value)
        }

        let mut new_node = self.rotate();
        let lh = get_height(&new_node.left);
        let rh = get_height(&new_node.right);
        new_node.update_height();
        *new_node
    }

    pub fn delete(&mut self, value: T) -> bool {
        // TODO implement me
        false
    }

    // Rotating a Node may modify the height of itself, child, grandchild and all of its parent node.
    // We make sure The height of parent node is correct by call the update_height recursively.
    fn rotate(mut self) -> Box<Avl<T>> {
        if self.balance_factor() > 1 {
            let left = self.left.take();
            match left {
                None => panic!("error"),
                Some(child) => {
                    if child.balance_factor() > 0 {
                        // without right node, move left to the original value
                        self.left.replace(child);
                        self.right_rotate()
                    } else {
                        // with right node
                        let left = child.left_rotate();
                        self.left.replace(left);
                        self.right_rotate()
                    }
                }
            }
        } else if self.balance_factor() < -1 {
            let right = self.right.take();
            match right {
                None => panic!("error"),
                Some(child) => {
                    // Right-leaning tree
                    if child.balance_factor() < 0 {
                        // put right back
                        self.right.replace(child);
                        self.left_rotate()
                    } else {
                        let right = child.right_rotate();
                        self.right.replace(right);
                        self.left_rotate()
                    }
                }
            }
        } else {
            Box::new(self)
        }
    }

    fn right_rotate(mut self) -> Box<Avl<T>> {
        let mut child = match self.left.take() {
            None => return Box::new(self),
            Some(node) => node
        };

        if child.right.is_none() {
            // In this case, the height of itself and its child might be modified
            child.right.replace(Box::new(self));
            util::update_height(&mut child.right);
            child.update_height();
            child
        } else {
            let grandchild = child.right.take();
            self.left.replace(grandchild.unwrap());
            self.update_height();
            child.right.replace(Box::new(self));
            child.update_height();
            child
        }
    }

    fn left_rotate(mut self) -> Box<Avl<T>> {
        let mut child = match self.right.take() {
            None => return Box::new(self),
            Some(node) => node
        };

        if child.left.is_none() {
            child.left.replace(Box::new(self));
            util::update_height(&mut child.left);
            child.update_height();
            child
        } else {
            let grandchild = child.left.take();
            self.right.replace(grandchild.unwrap());
            self.update_height();
            child.left.replace(Box::new(self));
            child.update_height();
            child
        }
    }

    fn update_height(&mut self) {
        use util::get_height;
        self.height = max(get_height(&self.left), get_height(&self.right)) + 1
    }

    fn balance_factor(&self) -> i32 {
        util::get_height(&self.left) - util::get_height(&self.right)
    }

    fn height(&self) -> i32 {
        self.height
    }
}

mod util {
    use crate::avl::{Avl, AvlBoxOption};
    use std::cmp::Ordering;
    use std::fmt::Debug;

    pub(crate) fn get_height<T: Ord + Debug>(node: &AvlBoxOption<T>) -> i32 {
        match node {
            None => -1,
            Some(node) => node.height,
        }
    }

    pub(crate) fn update_height<T: Ord + Debug>(node: &mut AvlBoxOption<T>) {
        match node {
            None => {}
            Some(node) => node.update_height(),
        }
    }

    pub(crate) fn insert<T: Ord + Debug>(root_opt: AvlBoxOption<T>, new_value: T) -> AvlBoxOption<T> {
        match root_opt {
            None => {
                Some(Avl::new(new_value))
            }
            Some(root) => {
                let mut new_root = Avl::insert(*root, new_value);
                let mut new_root = new_root.rotate();
                new_root.update_height();
                Some(new_root)
            }
        }
    }

    pub(crate) fn rotate<T: Ord + Debug>(root: AvlBoxOption<T>) -> AvlBoxOption<T> {
        match root {
            None => { None }
            Some(root) => {
                let mut root = root.rotate();
                root.update_height();
                Some(root)
            }
        }
    }

    pub(crate) fn search<T: Ord + Debug>(node: &AvlBoxOption<T>, value: &T) -> bool {
        match node {
            None => false,
            Some(n) => {
                match n.value.cmp(value) {
                    Ordering::Equal => true,
                    Ordering::Less => search(&n.right, value),
                    Ordering::Greater => search(&n.left, value),
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::avl::{display_avl, Avl};

    #[test]
    fn test_insert() {
        let root = Avl::new(0);
        assert_eq!(root.height, 0);
        assert_eq!(root.value, 0);
        // search_all(&root, vec![0]);

        let root = root.insert(1);
        assert_eq!(root.height, 1);
        assert_eq!(root.value, 0);
        // search_all(&root, vec![0, 1]);

        let mut root = root.insert(2);
        assert_eq!(root.height, 1);
        assert_eq!(root.value, 1);
        // search_all(&root, vec![0, 1, 2]);

        let mut root = root.insert(3);
        assert_eq!(root.height, 2);
        assert_eq!(root.value, 1);
        search_all(&root, vec![0, 1, 2, 3]);

        let root = root.insert(4);
        assert_eq!(root.height, 2);
        assert_eq!(root.value, 1);

        let root = root.insert(5);
        assert_eq!(root.height, 2);
        assert_eq!(root.value, 3);
        let root = root.insert(6);
        assert_eq!(root.height, 2);
        assert_eq!(root.value, 3);
        let root = root.insert(7);
        assert_eq!(root.height, 3);
        assert_eq!(root.value, 3);
        let root = root.insert(8);
        assert_eq!(root.height, 3);
        assert_eq!(root.value, 3);
        let root = root.insert(9);
        assert_eq!(root.height, 3);
        assert_eq!(root.value, 3);
        let root = root.insert(10);
        assert_eq!(root.height, 3);
        assert_eq!(root.value, 3);
        let root = root.insert(11);
        assert_eq!(root.height, 3);
        assert_eq!(root.value, 7);
    }

    #[test]
    fn test_update_height() {
        let mut zero = Avl::new(0);
        let mut one = Avl::new(1);
        let mut two = Avl::new(2);

        // 0
        // |
        // 1
        // |
        // 2

        zero.height = 2;
        // this is an erroneous value, just for test
        one.height = 0;
        two.height = 0;

        one.left.replace(two);
        zero.left.replace(one);

        let mut root = zero.right_rotate();
        root.update_height();
        assert_eq!(root.height(), 1);
    }

    #[test]
    fn test_right_rotation() {
        let zero = Avl::new(0);
        let mut one = Avl::new(1);
        let mut three = Avl::new(3);
        let mut four = Avl::new(4);
        let five = Avl::new(5);

        //        4
        //      /   \
        //     3     5
        //    /
        //   1
        //  /
        // 0
        one.left.replace(zero);
        three.left.replace(one);
        four.left.replace(three);
        four.right.replace(five);

        assert_eq!(four.value, 4);

        let three = four.left.unwrap();
        assert_eq!(three.value, 3);
        let five = four.right.unwrap();
        assert_eq!(five.value, 5);

        let one = three.left.unwrap();
        assert_eq!(one.value, 1);

        let zero = one.left.unwrap();
        assert_eq!(zero.value, 0);

        // reconstruct target tree
        let zero = Avl::new(0);
        let mut one = Avl::new(1);
        let mut three = Avl::new(3);
        let mut four = Avl::new(4);
        let five = Avl::new(5);

        //        4
        //      /   \
        //     3     5
        //    /
        //   1
        //  /
        // 0
        one.left.replace(zero);
        three.left.replace(one);
        four.left.replace(three);
        four.right.replace(five);

        let root = four.right_rotate();
        assert_eq!(root.value, 3);
        let one = root.left.unwrap();
        let four = root.right.unwrap();
        assert_eq!(one.value, 1);
        assert_eq!(four.value, 4);

        let zero = one.left.unwrap();
        let five = four.right.unwrap();
        assert_eq!(zero.value, 0);
        assert_eq!(five.value, 5);
    }

    #[test]
    fn test_left_rotation() {
        let zero = Avl::new(0);
        let mut one = Avl::new(1);
        let mut two = Avl::new(2);
        let mut three = Avl::new(3);
        let four = Avl::new(4);

        //    1
        //  /   \
        // 0     2
        //        \
        //         3
        //          \
        //           4
        three.right.replace(four);
        two.right.replace(three);
        one.left.replace(zero);
        one.right.replace(two);

        assert_eq!(one.value, 1);

        let zero = one.left.unwrap();
        assert_eq!(zero.value, 0);
        let two = one.right.unwrap();
        assert_eq!(two.value, 2);

        let three = two.right.unwrap();
        assert_eq!(three.value, 3);

        let four = three.right.unwrap();
        assert_eq!(four.value, 4);

        // reconstruct target tree
        let zero = Avl::new(0);
        let mut one = Avl::new(1);
        let mut two = Avl::new(2);
        let mut three = Avl::new(3);
        let four = Avl::new(4);

        //    1
        //  /   \
        // 0     2
        //        \
        //         3
        //          \
        //           4
        three.right.replace(four);
        two.right.replace(three);
        one.left.replace(zero);
        one.right.replace(two);

        let root = one.left_rotate();
        //        2
        //      /   \
        //     1     3
        //    /        \
        //   0          4
        assert_eq!(root.value, 2);

        let one = root.left.unwrap();
        let three = root.right.unwrap();
        assert_eq!(one.value, 1);
        assert_eq!(three.value, 3);

        let zero = one.left.unwrap();
        let four = three.right.unwrap();
        assert_eq!(zero.value, 0);
        assert_eq!(four.value, 4);
    }

    fn assert_option<T: Ord + std::fmt::Debug>(data: &Option<Box<Avl<T>>>, value: Option<T>) {
        match data {
            None => assert!(value.is_none()),
            Some(data) => assert_eq!(data.value, value.unwrap()),
        }
    }

    fn search_all(avl: &Avl<i32>, vec: Vec<i32>) {
        for v in vec {
            if !avl.search(&v) {
                panic!("search result not found for [{}]", v);
            }
        }
    }
}
