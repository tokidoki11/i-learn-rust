mod delete_earn;
mod longest_subsequence;
mod maximal_square;
mod min_cost;
mod multiplication;
mod robber;
mod tribonacci;

pub struct Solution {}

fn robber() {
    let nums = vec![1, 2, 1, 100, 1, 1];
    let result = Solution::rob(nums);
    println!("{}", result);
}

fn min_cost() {
    let nums = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    let result = Solution::min_cost_climbing_stairs(nums);
    println!("{}", result);
}

fn tribonacci() {
    let result = tribonacci::Solution::tribonacci(25);
    println!("{}", result);
}

fn delete_earn() {
    let nums = vec![2, 2, 3, 3, 3, 4];
    let result = Solution::delete_and_earn(nums);
    println!("{}", result);
}

fn multiplication() {
    let nums = vec![-5, -3, -3, -2, 7, 1];
    let multipliers = vec![-10, -5, 3, 4, 6];
    let result = Solution::maximum_score(nums, multipliers);
    println!("{}", result);
}

fn longest_subsequence() {
    let text1 = String::from("abcde");
    let text2 = String::from("ace");
    let result = Solution::longest_common_subsequence(text1, text2);
    println!("{}", result);
}

fn maximal_square() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    let result = Solution::maximal_square(matrix);
}

fn main() {
    // robber();
    // min_cost()
    // delete_earn();
    // tribonacci();
    // multiplication();
    // longest_subsequence();
    maximal_square();
}
