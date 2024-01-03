pub use super::Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut tabular: Vec<i32> = vec![0, 1, 1];

        for i in 3..=n {
            let result = tabular.get((i - 1) as usize).unwrap()
                + tabular.get((i - 2) as usize).unwrap()
                + tabular.get((i - 3) as usize).unwrap();

            tabular.push(result)
        }

        return tabular.get(n as usize).unwrap().clone();
    }
}
