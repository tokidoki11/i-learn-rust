use std::cmp::max;
use std::collections::HashMap;

use super::Solution;

/*
// 1. define dp function
dp(n)
// 2. define reoccuring
if taking n, then n-1 and n-2 are not allowed, vice versa
dp(n) = Max(nums[n], dp(n-1) + dp(n + 2))
robber function similar
// 3. define base case
dp(0) = nums[0] // only 1 option
dp(1) = max(nums[0],nums[1]) // if there is only  2 options
*/

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        // let mut memoized: HashMap<i32, i32> = HashMap::new();
        let nums = Self::max_map(nums);
        // Self::dp_top_bottom(nums.len() as i32, &nums, &mut memoized)
        Self::dp_bottom_top(&nums)
    }

    fn max_map(nums: Vec<i32>) -> Vec<i32> {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            let count = hash_map.get(&num).cloned().unwrap_or_default();
            hash_map.insert(num, count + 1);
        }

        let size: usize = (hash_map.keys().max().unwrap() + 1) as usize;
        let mut result: Vec<i32> = vec![0; size];

        for (&key, &value) in hash_map.iter() {
            result[key as usize] = key * value;
        }

        result
    }

    fn dp_top_bottom(i: i32, nums: &Vec<i32>, hash_map: &mut HashMap<i32, i32>) -> i32 {
        if i == 0 {
            return nums.get(0).cloned().unwrap_or_default();
        }
        if i == 1 {
            return max(
                nums.get(0).cloned().unwrap_or_default(),
                nums.get(1).cloned().unwrap_or_default(),
            );
        }
        if !hash_map.contains_key(&i) {
            let current = nums.get(i as usize).cloned().unwrap_or_default()
                + Self::dp_top_bottom(i - 2, nums, hash_map);
            let other = Self::dp_top_bottom(i - 1, nums, hash_map);
            hash_map.insert(i, max(current, other));
        }

        hash_map.get(&i).cloned().unwrap()
    }

    fn dp_bottom_top(nums: &Vec<i32>) -> i32 {
        let mut tabular: Vec<i32> = vec![0; nums.len()];
        tabular[0] = nums[0];
        tabular[1] = nums[1].max(nums[0]);

        for i in 2..nums.len() {
            tabular[i] = (nums[i] + tabular[i - 2]).max(tabular[i - 1]);
        }

        tabular.last().cloned().unwrap()
    }
}
