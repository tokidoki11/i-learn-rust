use std::cmp::max;
use std::collections::HashMap;

use super::Solution;

/**
 *
 * fn - dp(idx, currentNumber)
 * reocurring
 * from idx+1 to end (n), if nums(n) > currentNumber run 1 + dp(n, nums(n))
 * get max from list
 *
 * base condition
 * if(idx = nums.len() -1) return
 *
 *
 */
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut memo: HashMap<(usize, i32), i32> = HashMap::new();
        let result = Self::length_of_lis_dp(0, nums[0], &nums, &mut memo);
        println!("{:?}", memo);
        result
    }
    fn length_of_lis_dp(
        idx: usize,
        current_number: i32,
        nums: &Vec<i32>,
        memo: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if idx == nums.len() - 1 {
            return if nums[idx - 1] < nums[idx] { 1 } else { 0 };
        }

        if !memo.contains_key(&(idx, current_number)) {
            let mut count = 1;
            for i in idx + 1..nums.len() {
                let next_number = nums[i];
                if current_number < next_number {
                    let take = 1 + Self::length_of_lis_dp(i, next_number, nums, memo);
                    let skip = Self::length_of_lis_dp(i, current_number, nums, memo);
                    let next = max(take, skip);
                    count = max(count, next);
                } else {
                    let take = Self::length_of_lis_dp(i, next_number, nums, memo);
                    let skip = Self::length_of_lis_dp(i, current_number, nums, memo);
                    let next = max(take, skip);
                    count = max(count, next);
                }
            }
            memo.insert((idx, current_number), count);
        }

        memo.get(&(idx, current_number)).unwrap().clone()
    }
}

mod test {

    use super::*;

    #[test]
    fn test_length_of_lis_1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let result = Solution::length_of_lis(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_length_of_lis_2() {
        let nums = vec![7, 7, 7, 7];
        let result = Solution::length_of_lis(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_length_of_lis_3() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        let result = Solution::length_of_lis(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_length_of_lis_4() {
        let nums = vec![0];
        let result = Solution::length_of_lis(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_length_of_lis_5() {
        let nums = vec![4, 10, 4, 3, 8, 9];
        let result = Solution::length_of_lis(nums);
        assert_eq!(result, 3);
    }
}
