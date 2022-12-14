mod easy;
mod hard;
mod medium;

#[cfg(test)]
mod easy_tests {
    use super::easy::two_sum::two_sum;
    #[test]
    fn two_sum_test() {
        let case1 = vec![2, 7, 11, 15];
        let result1 = two_sum(case1, 9);
        assert_eq!(result1, vec![0, 1]);

        let case2 = vec![3, 2, 4];
        let result1 = two_sum(case2, 6);
        assert_eq!(result1, vec![1, 2]);

        let case3 = vec![3, 3];
        let result1 = two_sum(case3, 6);
        assert_eq!(result1, vec![0, 1]);
    }

    use super::easy::longest_common_prefix::longest_common_prefix;
    #[test]
    fn longest_common_prefix_test() {
        let case1 = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        assert_eq!(longest_common_prefix(case1), String::from("fl"))
    }

    use super::easy::roman_to_int::roman_to_int;
    #[test]
    fn roman_to_int_test() {
        let case1 = String::from("MCMXCIV");
        assert_eq!(roman_to_int(case1), 1994);

        let case2 = String::from("III");
        assert_eq!(roman_to_int(case2), 3);
    }
}

mod medium_tests {
    use super::medium::add_two_numbers::{add_two_numbers, ListNode};
    #[test]
    fn add_two_numbers_test() {
        let l1 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode { next: None, val: 9 })),
                val: 9,
            })),
            val: 1,
        }));

        let l2 = Some(Box::new(ListNode { next: None, val: 9 }));

        let exp1 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode { next: None, val: 1 })),
                    val: 0,
                })),
                val: 0,
            })),
            val: 0,
        }));

        assert_eq!(add_two_numbers(l1, l2), exp1);
    }
}

mod hard_tests {

    #[test]
    fn it_works() {
        let a = 2;
        assert_eq!(a, 2);
    }
}
