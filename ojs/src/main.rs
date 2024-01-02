mod min_cost;
mod robber;

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

fn main() {
    // robber();
    min_cost()
}
