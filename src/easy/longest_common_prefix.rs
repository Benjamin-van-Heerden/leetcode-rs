use std::ops::Add;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        if s.starts_with(&prefix) {
            continue;
        } else {
            let mut new_prefix = String::new();
            for (cp, cs) in prefix.chars().zip(s.chars()) {
                if cp == cs {
                    new_prefix.push_str(&cs.to_string())
                } else {
                    break;
                }
            }
            prefix = new_prefix;
        }
    }

    prefix
}
