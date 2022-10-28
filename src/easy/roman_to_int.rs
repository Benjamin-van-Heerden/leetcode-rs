use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let conversions = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut result = 0;
    let mut skip = false;
    for (c1, c2) in s.chars().zip(s.chars().skip(1)) {
        if skip {
            skip = false;
            continue;
        }
        let temp_res: i32 = match (c1, c2) {
            ('I', 'V' | 'X') => {
                skip = true;
                conversions[&c2] - conversions[&c1]
            }
            ('X', 'L' | 'C') => {
                skip = true;
                conversions[&c2] - conversions[&c1]
            }
            ('C', 'D' | 'M') => {
                skip = true;
                conversions[&c2] - conversions[&c1]
            }
            (_, _) => conversions[&c1],
        };
        result += temp_res;
    }
    if !skip {
        result += conversions[&s.chars().last().unwrap()];
    }
    result
}
