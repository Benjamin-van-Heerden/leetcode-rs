use std::cmp;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn traverse(ln: Option<Box<ListNode>>, accum: &mut Vec<i32>) -> Vec<i32> {
        match ln {
            Some(l) => {
                accum.push(l.val);
                return traverse(l.next, accum);
            }
            None => return accum.to_vec(),
        }
    }

    let mut l1_accum: Vec<i32> = Vec::new();
    let mut l1_nums = traverse(l1, &mut l1_accum);

    let mut l2_accum: Vec<i32> = Vec::new();
    let mut l2_nums = traverse(l2, &mut l2_accum);

    let longest_num = cmp::max(l1_nums.len(), l2_nums.len());

    for _pad_zeros in 0..(longest_num - l1_nums.len()) {
        l1_nums.push(0);
    }

    for _pad_zeros in 0..(longest_num - l2_nums.len()) {
        l2_nums.push(0);
    }

    let mut result_vec: Vec<i32> = Vec::new();
    let mut spillover_one = 0;
    for (a, b) in l1_nums.iter().zip(l2_nums.iter()) {
        let mut res_vec_val = a + b + spillover_one;
        spillover_one = 0;
        if res_vec_val > 9 {
            spillover_one = 1;
            res_vec_val = res_vec_val % 10
        }
        result_vec.push(res_vec_val)
    }
    if spillover_one == 1 {
        result_vec.push(1);
    }
    result_vec.reverse();

    let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: result_vec[0],
        next: None,
    }));
    for val in result_vec[1..].to_vec() {
        result = Some(Box::new(ListNode { val, next: result }));
    }
    result
}

pub fn add_two_numbers_best(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;
    let mut p = l1;
    let mut q = l2;

    let mut carry = 0;

    while p.is_some() || q.is_some() {
        let sum = match (&p, &q) {
            (Some(x), Some(y)) => x.val + y.val + carry,
            (Some(x), None) => x.val + carry,
            (None, Some(y)) => y.val + carry,
            (None, None) => carry,
        };

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        p = if p.is_some() { p.unwrap().next } else { p };
        q = if p.is_some() { q.unwrap().next } else { q };
    }
    if carry > 0 {
        current.next = Some(Box::new(ListNode::new(carry)));
    }

    dummy_head.next
}

pub fn add_two_numbers_fail_overflow(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn traverse(ln: Option<Box<ListNode>>, accum: &mut Vec<i32>) -> Vec<i32> {
        match ln {
            Some(l) => {
                accum.push(l.val);
                return traverse(l.next, accum);
            }
            None => return accum.to_vec(),
        }
    }

    let mut l1_accum: Vec<i32> = Vec::new();
    let l1_nums = traverse(l1, &mut l1_accum);
    let mut l1_val: i64 = 0;
    for (idx, val) in l1_nums.iter().enumerate() {
        let pow = idx as u32;
        l1_val += (*val as i64) * 10_i64.pow(pow);
    }

    let mut l2_accum: Vec<i32> = Vec::new();
    let l2_nums = traverse(l2, &mut l2_accum);
    let mut l2_val: i64 = 0;
    for (idx, val) in l2_nums.iter().enumerate() {
        let pow = idx as u32;
        l2_val += (*val as i64) * 10_i64.pow(pow);
    }

    let res_val: String = (l1_val + l2_val).to_string();
    let rev = res_val.chars().collect::<String>();

    let ans = rev
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: ans[0],
        next: None,
    }));
    for val in ans[1..].to_vec() {
        result = Some(Box::new(ListNode { val, next: result }));
    }
    result
}
