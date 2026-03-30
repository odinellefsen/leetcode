// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn push_left_spine(mut curr: Node, stack: &mut Vec<Rc<RefCell<TreeNode>>>) {
        while let Some(node) = curr {
            let left = node.borrow().left.clone();
            stack.push(node);
            curr = left;
        }
    }

    pub fn inorder_traversal(root: Node) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        Self::push_left_spine(root, &mut stack);
        let _ = &mut result;

        // TODO: implement inorder traversal.
        result
    }
}
