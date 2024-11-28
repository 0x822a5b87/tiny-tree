//test range query in a binary tree
struct RangeQueryTree {
    val: i32,
    left: Option<Box<RangeQueryTree>>,
    right: Option<Box<RangeQueryTree>>,
}

impl RangeQueryTree {
    fn new(new_val: i32, left: Option<RangeQueryTree>, right: Option<RangeQueryTree>) -> RangeQueryTree {
        let left = match left {
            None => None,
            Some(data) => Some(Box::new(data))
        };

        let right = match right {
            None => None,
            Some(data) => Some(Box::new(data))
        };
        RangeQueryTree {
            val: new_val,
            left,
            right,
        }
    }

    pub fn range_query(&self, start: i32, end: i32) -> Vec<i32> {
        let mut v = vec![];
        Self::inner_range_query(self, start, end, &mut v);
        v
    }

    fn inner_range_query(root: &RangeQueryTree, start: i32, end: i32, v: &mut Vec<i32>) {
        if start <= root.val && root.val <= end {
            v.push(root.val);
            let left = match &root.left {
                None => return,
                Some(v) => v.as_ref(),
            };
            Self::inner_range_query(left, start, end, v);

            let right = match &root.right {
                None => return,
                Some(v) => v.as_ref(),
            };
            Self::inner_range_query(right, start, end, v);
        }

        if root.val < start {
            let right = match &root.right {
                None => return,
                Some(v) => v.as_ref(),
            };
            Self::inner_range_query(right, start, end, v);
        }

        if root.val > end {
            let left = match &root.left {
                None => return,
                Some(v) => v.as_ref(),
            };
            Self::inner_range_query(left, start, end, v);
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::RangeQueryTree;

    #[test]
    fn test_insert() {
        //             4
        //           /    \
        //          2      6
        //         / \    / \
        //        1   3  5   7
        let root = RangeQueryTree::new(
            4,
            Some(
                RangeQueryTree::new(
                    2,
                    Some(RangeQueryTree::new(1, None, None)),
                    Some(RangeQueryTree::new(3, None, None)),
                ),
            ),
            Some(
                RangeQueryTree::new(
                    6,
                    Some(RangeQueryTree::new(5, None, None)),
                    Some(RangeQueryTree::new(7, None, None)),
                )
            ),
        );

        let vec = root.range_query(3, 5);
        assert_eq!(vec, vec![4, 3, 5]);
        let vec = root.range_query(0, 100);
        assert_eq!(vec, vec![4, 2, 1, 3, 6, 5, 7]);
    }
}