mod robber;

fn knapsack() {
    let nums = vec![1, 2, 3, 1];
    let result = robber::Solution::rob(nums);
    println!("{}", result);
}

fn main() {
    knapsack();
}
