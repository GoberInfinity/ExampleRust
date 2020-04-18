use std::cell::RefCell;
use std::rc::Rc;

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

fn main(){
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut root = Some(Rc::new(RefCell::new(TreeNode{
        val : 0,
        left : left,
        right : None,
    })));

    println!("{:?}", root);
}