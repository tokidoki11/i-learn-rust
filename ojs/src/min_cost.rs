use std::cmp::min;
use std::collections::HashMap;
pub struct Solution {}

/*
1. define dp function
    dp(i: i32)
2. define reoccuring
    taking on step i, reqires the cost + the cost from last step (i-1 or i-2)
    dp(i) = cost(i) + min(dp(i-1), dp(i-2))
3. define base case
   first step is either index 0 or index 1
   therefore dp(0) = cost(0) and dp(1) = cost(1)

note: top means n, not n-1

 */
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // let mut hash_map: HashMap<i32, i32> = HashMap::new();
        // return Self::top_down_dp(cost.len() as i32, &cost, &mut hash_map);
        Self::bottom_up_dp(cost.len() as i32, &cost)
    }

    fn top_down_dp(i: i32, cost: &Vec<i32>, hash_map: &mut HashMap<i32, i32>) -> i32 {
        match i {
            0 => *cost.get(0).unwrap(),
            1 => *cost.get(1).unwrap(),
            index => {
                if !hash_map.contains_key(&index) {
                    let result = *cost.get(index as usize).unwrap_or(&0)
                        + min(
                            Self::top_down_dp(index - 1, cost, hash_map),
                            Self::top_down_dp(index - 2, cost, hash_map),
                        );
                    hash_map.insert(index, result);
                }

                *hash_map.get(&index).unwrap()
            }
        }
    }

    fn bottom_up_dp(i: i32, cost: &Vec<i32>) -> i32 {
        let mut tabular: Vec<i32> =
            vec![cost.get(0).unwrap().clone(), cost.get(1).unwrap().clone()];
        tabular.resize((i + 1) as usize, 0);

        for step in 2..=i {
            tabular[step as usize] = cost.get(step as usize).unwrap_or(&0)
                + min(tabular[(step - 1) as usize], tabular[(step - 2) as usize]);
        }

        return tabular.get(i as usize).unwrap().clone();
    }

    // logic fail, not dp
    // [0,2,2,1]
    // fn count(cost: &Vec<i32>, idx: i32, last: Option<i32>) -> i32 {
    //     println!("idx: {}, last: {:?}", idx, last);
    //     if idx < 0 {
    //         return last.unwrap() + cost.get(idx as usize).unwrap_or(&0);
    //     }
    //     if idx - 2 < 0 {
    //         return last.unwrap();
    //     }

    //     let is_one_smaller_cost =
    //         cost.get((idx - 1) as usize).unwrap() < cost.get((idx - 2) as usize).unwrap();

    //     let step: i32 = if is_one_smaller_cost { 1 } else { 2 };
    //     let next_index = (idx - step);
    //     let total = Some(last.unwrap_or(0) + cost.get(next_index as usize).unwrap_or(&0));
    //     return Self::count(cost, next_index, total);
    // }
}
