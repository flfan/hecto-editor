// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    if let Some(node) = root {
        res.append(&mut inorder_traversal(node.borrow_mut().left.take()));
        res.push(node.borrow().val);
        res.append(&mut inorder_traversal(node.borrow_mut().right.take()));
    }
    res
}

pub fn level_order1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();

    if root.is_none() {
        return res;
    }
    let mut queue = vec![root.unwrap()];
    while !queue.is_empty() {
        let mut temp_arr = Vec::<i32>::new();
        let mut lens = queue.len();
        while lens != 0 {
            let current_node = queue.remove(0);
            temp_arr.push(current_node.borrow().val);
            if current_node.borrow().left.is_some() {
                queue.push(current_node.borrow_mut().left.take().unwrap())
            }
            if current_node.borrow().right.is_some() {
                queue.push(current_node.borrow_mut().right.take().unwrap())
            }
            lens = lens - 1;
        }
        res.push(temp_arr)
    }
    res
}

fn main() {}
