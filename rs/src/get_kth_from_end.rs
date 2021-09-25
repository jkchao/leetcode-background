
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

#[allow(dead_code)]
pub fn get_kth_from_end_1(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut f = head.as_ref();
    let mut s = head.as_ref();

    for _ in 0..k {
        f = f.unwrap().next.as_ref()
    }

    while f.is_some() {
        s = s.unwrap().next.as_ref();
        f = f.unwrap().next.as_ref();
    }

    return s.cloned();
}
