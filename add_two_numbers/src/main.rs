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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);

        let mut result = Some(Box::new(ListNode::new(0)));
        let mut current = result.as_mut();

        while l1.is_some() || l2.is_some() {
            if let Some(node) = l1 {
                current.as_mut().unwrap().val += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                current.as_mut().unwrap().val += node.val;
                l2 = node.next;
            }

            if let Some(node) = current {
                if node.val >= 10 {
                    node.val -= 10;
                    node.next = Some(Box::new(ListNode::new(1)));
                } else if l1.is_some() || l2.is_some() {
                    node.next = Some(Box::new(ListNode::new(0)));
                }

                current = node.next.as_mut();
            }
        }

        result
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None,
            })),
        })),
    }));

    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })),
    }));

    let result = Solution::add_two_numbers(l1, l2);
    let mut current = &result;
    while let Some(node) = current {
        println!("{}", node.val);
        current = &node.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));

        let result = Solution::add_two_numbers(l1, l2);
        let mut current = &result;
        let mut result = Vec::new();
        while let Some(node) = current {
            result.push(node.val);
            current = &node.next;
        }
        assert_eq!(result, vec![7, 0, 8]);
    }
}
