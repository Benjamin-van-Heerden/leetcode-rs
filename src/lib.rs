use easy::two_sum::two_sum;

mod easy;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_basic() {
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
}
