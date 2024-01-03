mod delete_earn;
mod longest_subsequence;
mod min_cost;
mod multiplication;
mod robber;
mod tribonacci;

fn robber() {
    let nums = vec![1, 2, 1, 100, 1, 1];
    let result = robber::Solution::rob(nums);
    println!("{}", result);
}

fn min_cost() {
    let nums = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    let result = min_cost::Solution::min_cost_climbing_stairs(nums);
    println!("{}", result);
}

fn tribonacci() {
    let result = tribonacci::Solution::tribonacci(25);
    println!("{}", result);
}

fn delete_earn() {
    let nums = vec![2, 2, 3, 3, 3, 4];
    let result = delete_earn::Solution::delete_and_earn(nums);
    println!("{}", result);
}

fn multiplication() {
    let nums = vec![-5, -3, -3, -2, 7, 1];
    let multipliers = vec![-10, -5, 3, 4, 6];
    let result = multiplication::Solution::maximum_score(nums, multipliers);
    println!("{}", result);
}

fn longest_subsequence() {
    let text1 = String::from("abcde");
    let text2 = String::from("ace");
    let result = longest_subsequence::Solution::longest_common_subsequence(text1, text2);
    println!("{}", result);
}

fn main() {
    // robber();
    // min_cost()
    // delete_earn();
    // tribonacci();
    // multiplication();
    longest_subsequence();
}
