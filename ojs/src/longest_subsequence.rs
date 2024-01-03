use std::cmp::max;

use super::Solution;

//https://www.enjoyalgorithms.com/blog/longest-common-subsequence
// tle
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // let mut memo = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        // let m = text1.len() as i32;
        // let n = text2.len() as i32;
        // Self::lcs(&text1, &text2, m - 1, n - 1, &mut memo)

        Self::buttom_up(text1, text2)
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

    // from front (plusing) (not allowed - middle table is supposedly blank)
    fn buttom_up(text1: String, text2: String) -> i32 {
        let mut memo = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        let m = text1.len();
        let n = text2.len();

        for i in 1..=m {
            for j in 1..=n {
                let c1 = text1.as_bytes()[i - 1];
                let c2 = text2.as_bytes()[j - 1];
                if c1 == c2 {
                    memo[i][j] = 1 + memo[i - 1][j - 1]
                } else {
                    memo[i][j] = max(memo[i - 1][j], memo[i][j - 1])
                }
            }
        }

        memo[m][n]
    }
}
