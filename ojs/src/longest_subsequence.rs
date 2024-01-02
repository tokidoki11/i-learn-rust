use std::cmp::max;

pub struct Solution {}

//https://www.enjoyalgorithms.com/blog/longest-common-subsequence
// tle
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut memo = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        let m = text1.len() as i32;
        let n = text2.len() as i32;
        Self::lcs(&text1, &text2, m - 1, n - 1, &mut memo)
    }

    fn lcs(X: &str, Y: &str, m: i32, n: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
        if m < 0 || n < 0 {
            return 0;
        }

        if X.chars().nth(m as usize) == Y.chars().nth(n as usize) {
            return 1 + Self::lcs(X, Y, m - 1, n - 1, memo);
        }

        if memo[m as usize][n as usize] == 0 {
            memo[m as usize][n as usize] = max(
                Self::lcs(X, Y, m, n - 1, memo),
                Self::lcs(X, Y, m - 1, n, memo),
            );
        }
        memo[m as usize][n as usize]
    }
}
