use std::cmp::max;
use std::collections::HashMap;

pub struct Solution {}

// lesson
// 1. define dp function
// 2. define reoccuring (line 21 -> on i, rob or not rob)
// 3. define base case  (line 18 | 19 -> 1 house then rob, 2 houses then rob the max)
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        Self::dp((nums.len() - 1) as i32, &nums, &mut hash_map)
    }

    fn dp(i: i32, nums: &Vec<i32>, hash_map: &mut HashMap<i32, i32>) -> i32 {
        match i {
            0 => *nums.get(0).unwrap(),
            1 => *max(nums.get(0).unwrap(), nums.get(1).unwrap()),
            index => {
                if !hash_map.contains_key(&index) {
                    let result = max(
                        Self::dp(index - 2, nums, hash_map) + *nums.get((index) as usize).unwrap(),
                        Self::dp(index - 1, nums, hash_map),
                    );
                    hash_map.insert(index, result);
                }
                *hash_map.get(&index).unwrap()
            }
        }
    }
}
