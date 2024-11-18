struct Solution {}


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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {

    pub fn build_tree(values: Vec<Option<i32>>) {
      
      for value in values {

        
        
      }

    }

    pub fn print(root: Option<Rc<RefCell<TreeNode>>>) {
          
    }
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        vec![0]
    }
}


fn main() {
  Solution::build_tree( 
    vec![
      Some(3),
      Some(5),
      Some(1),
      Some(6),
      Some(2),
      Some(0),
      Some(8),
      None,
      None,
      Some(7),
      Some(4)
    ]
  );
}
