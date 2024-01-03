use super::Solution;

// note: result will always 0 | n^2 > n is int
// fn dp(m,n) -> state m,n
// reoccuring:
// if tile '1' = 1 + min(dp(m-1,n-1) + dp(m-1,n) + dp(m,n-1)) -< min because if there is 0, it wont create a square
// [0,1,1]
// [1,1,1]
// [1,1,0]
// else ??
// base condition:
// dp(0,0) = if '1' then 1 else 0 -> border all the same
// m = 0 | n = 0 -> if '1' then 1 else 0

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut memo = vec![vec![-1; n]; m];
        for i in 0..m {
            memo[i][0] = if matrix[i][0] == '1' { 1 } else { 0 };
        }
        for j in 0..n {
            memo[0][j] = if matrix[0][j] == '1' { 1 } else { 0 };
        }

        Self::max_square_dp(m - 1, n - 1, &matrix, &mut memo);
        return memo.iter().flatten().max().unwrap().pow(2);
    }

    fn max_square_dp(m: usize, n: usize, matrix: &Vec<Vec<char>>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if m == 0 || n == 0 || memo[m][n] != -1 {
            return memo[m][n];
        }

        memo[m - 1][n - 1] = Self::max_square_dp(m - 1, n - 1, matrix, memo);
        memo[m][n - 1] = Self::max_square_dp(m, n - 1, matrix, memo);
        memo[m - 1][n] = Self::max_square_dp(m - 1, n, matrix, memo);
        let next = vec![memo[m - 1][n - 1], memo[m][n - 1], memo[m - 1][n]];

        if memo[m][n] == -1 {
            memo[m][n] = 0;
            if matrix[m][n] == '1' {
                memo[m][n] = 1 + next.iter().min().unwrap().clone();
            }
        }

        return memo[m][n];
    }
}
