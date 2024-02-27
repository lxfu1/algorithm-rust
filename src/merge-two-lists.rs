/**
 * 将两个升序链表合并为⼀个新的升序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
 * 示例：
 * 输⼊：1->2->4, 1->3->4
 * 输出：1->1->2->3->4->4
*/

pub struct Node {
  val: Option<i32>,
  next: Option<Box<Node>>,
}

impl Node {
  pub fn new(val: Option<i32>) -> Self {
    Node { val, next: None }
  }
  pub fn last(&mut self) -> &mut Self {
    let mut current: &mut Node = self;
    while let Some(ref mut next) = current.next {
      current = next;
    }
    current
  }
  pub fn append(&mut self, val: Option<i32>) -> &mut Self {
    let new_node: Node = Node::new(val);
    let last = self.last();
    last.next = Some(Box::new(new_node));
    self
  }
}

pub fn merge_two_lists(l1: Option<Box<Node>>, l2: Option<Box<Node>>) -> Option<Box<Node>> {
  match (l1, l2) {
    (None, None) => None,
    (Some(node), None) => Some(node),
    (None, Some(node)) => Some(node),
    (Some(mut node1), Some(mut node2)) => {
      if node1.val <= node2.val {
        node1.next = merge_two_lists(node1.next, Some(node2));
        Some(node1)
      } else {
        node2.next = merge_two_lists(Some(node1), node2.next);
        Some(node2)
      }
    }
  }
}

fn print_list(mut list: Option<Box<Node>>) {
  while let Some(node) = list {
    print!("{:?} ", node.val);
    list = node.next;
  }
}

fn main() {
  let mut l1 = Node::new(Some(1));
  l1.append(Some(2)).append(Some(4));
  let mut l2 = Node::new(Some(1));
  l2.append(Some(3)).append(Some(4));
  let merge_list: Option<Box<Node>> = merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));
  print_list(merge_list)
}
