use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, usize> = HashMap::new();
    let mut answer: Vec<i32> = Vec::new();

    for (i, num) in nums.iter().enumerate() {
        let t = target - num;
        m.insert(t, i);
    }

    for (i, num) in nums.iter().enumerate() {
        if m.contains_key(num) {
            let i1 = m[num] as i32;
            let i2 = i as i32;
            if i1 == i2 {
                continue;
            } else if i1 < i2 {
                answer.push(i1);
                answer.push(i2);
                break;
            } else {
                answer.push(i2);
                answer.push(i1);
                break;
            }
        }
    }
    answer
}
