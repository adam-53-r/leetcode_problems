struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None };
        let mut head = head;
        let mut current = head.as_mut().unwrap();

        while let Some(next) = current.next.as_mut() {
            if current.val == next.val {
                current.next = next.next.take();
            }
            else {
                current = current.next.as_mut().unwrap();                
            }
        }
        head
    }
}


fn main() {

    let mut list = Some(Box::new(ListNode::new(1)));

    let mut current = list.as_mut().unwrap();
    current.next = Some(Box::new(ListNode::new(1)));

    current = current.next.as_mut().unwrap();
    current.next = Some(Box::new(ListNode::new(2)));

    current = current.next.as_mut().unwrap();
    current.next = Some(Box::new(ListNode::new(3)));

    current = current.next.as_mut().unwrap();
    current.next = Some(Box::new(ListNode::new(3)));

    println!("{:?}", Solution::delete_duplicates(list));
}
