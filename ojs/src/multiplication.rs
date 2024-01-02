use std::cmp::max;

pub struct Solution {}

// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut memoize: Vec<Vec<i32>> = vec![vec![i32::MIN; nums.len()]; multipliers.len()];
        // let a = Self::top_bottom_dp(0, 0, &nums, &multipliers, &mut memoize);
        // print!("{:?}", memoize);
        let a = Self::bottom_top_dp(&nums, &multipliers);
        return a;
    }

    // not working
    fn greedy(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        // greedy
        let mut total = 0;
        let mut nums = nums.clone();

        for multiplier in multipliers {
            let left = nums.first().unwrap().clone() * multiplier;
            let right = nums.last().unwrap().clone() * multiplier;

            if (left > right) {
                total += left;
                nums.remove(0);
            } else {
                total += right;
                nums.pop();
            }
        }

        total
    }

    fn top_bottom_dp(
        m: i32,
        left_idx: i32,
        nums: &Vec<i32>,
        multipliers: &Vec<i32>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if m == multipliers.len() as i32 {
            return 0;
        }

        let multiplier = multipliers[m as usize];

        if memo[m as usize][left_idx as usize] == i32::MIN {
            let right_idx = (nums.len() as i32 - 1) - (m - left_idx);
            let left = nums[left_idx as usize] * multiplier
                + Self::top_bottom_dp(m + 1, left_idx + 1, nums, multipliers, memo);
            let right = nums[right_idx as usize] * multiplier
                + Self::top_bottom_dp(m + 1, left_idx, nums, multipliers, memo);

            memo[m as usize][left_idx as usize] = max(left, right);
        }

        return memo[m as usize][left_idx as usize];
    }

    fn bottom_top_dp(nums: &Vec<i32>, multipliers: &Vec<i32>) -> i32 {
        // We also need to initialize dpdp with one extra row so that we don't go out of bounds in the first iteration of the outer loop.
        let mut memo: Vec<Vec<i32>> = vec![vec![0; nums.len() + 1]; multipliers.len() + 1];
        // base info dp[multipliers.len()][*] = 0 fullfilled

        for m in (0..multipliers.len()).rev() {
            let multiplier = multipliers[m];
            for left_idx in (0..=m).rev() {
                let right_idx = nums.len() - 1 - (m - left_idx);
                memo[m][left_idx] = max(
                    multiplier * nums[left_idx] + memo[m + 1][left_idx + 1],
                    multiplier * nums[right_idx] + memo[m + 1][left_idx],
                );
            }
        }
        return memo[0][0];
    }
}
