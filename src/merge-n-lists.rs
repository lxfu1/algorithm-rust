// 合并 k 个排序链表，返回合并后的排序链表。请分析和描述算法的复杂度。
// 示例:
// 输⼊:
// [
//   1->4->5,
//   1->3->4,
//   2->6
// ]
// 输出: 1->1->2->3->4->4->5->6

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

pub fn merge_k_lists(lists: Vec<Option<Box<Node>>>) -> Option<Box<Node>> {
  let mut res = None;
  for list in lists {
    res = merge_two_lists(res, list);
  }
  res
}

pub fn print_list(mut list: Option<Box<Node>>) {
  while let Some(node) = list {
    print!("{:?} ", node.val);
    list = node.next;
  }
}

pub fn mock_node(list: Vec<Vec<i32>>) -> Vec<Option<Box<Node>>> {
  let mut res: Vec<Option<Box<Node>>> = vec![];
  for i in list {
    let mut node = Node::new(None);
    for j in i {
      node.append(Some(j));
    }
    res.insert(0, node.next);
  }
  res
}

fn main() {
  let mock: Vec<Vec<i32>> = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
  let lists = mock_node(mock);
  let merged_list = merge_k_lists(lists);
  print_list(merged_list);
}
