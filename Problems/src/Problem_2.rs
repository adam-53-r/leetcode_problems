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


struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

        if l1.is_none() && l2.is_none() {
            return None;
        }

        let mut out: Box<ListNode> = Box::new(ListNode::new(0));
        let mut tail = &mut out;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let val = match l1 {
                Some(a) => {l1 = a.next; a.val},
                None => {0}
            } +
            match l2 {
                Some(b) => {l2 = b.next; b.val},
                None => {0}
            }
            + carry;
            carry = val / 10;
            tail.next = Some(Box::new(ListNode::new(val%10)));
            tail = tail.next.as_mut().unwrap();
        }

        out.next
    }
}

fn main() {
    let mut l1 = Box::new(ListNode::new(2));
    let mut l12 = Box::new(ListNode::new(4));
    let l13 = Box::new(ListNode::new(3));
    l12.next = Some(l13);
    l1.next = Some(l12);

    let mut l2 = Box::new(ListNode::new(5));
    let mut l22 = Box::new(ListNode::new(6));
    let l23 = Box::new(ListNode::new(4));
    l22.next = Some(l23);
    l2.next = Some(l22);

    println!("{:?}", Solution::add_two_numbers(Some(l1), Some(l2)));
}
