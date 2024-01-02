mod knapsack;

fn knapsack() {
    let nums = vec![1, 2, 3, 1];
    let result = knapsack::Solution::rob(nums);
    println!("{}", result);
}

fn main() {
    knapsack();
}
