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

fn main(){
    let node_0 = Some(Box::new(ListNode::new(0)));
    let node_1 = Some(Box::new(ListNode{
        val:1,
        next: node_0
    }));

    println!("{:?}", node_1);
}