use super::Solution;
/*
state:
remaining -> remaining amount

recursion:
next_amount = remaining - coin[index]
smallest = min()
*/

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut memo = vec![std::i32::MAX; (amount + 1) as usize];
        Self::coin_change_dp(amount, &coins, &amount, &mut memo)
    }

    fn coin_change_dp(remaining: i32, coins: &Vec<i32>, amount: &i32, memo: &mut Vec<i32>) -> i32 {
        if remaining == 0 {
            return 0;
        }

        if memo[remaining as usize] == std::i32::MAX {
            let mut calc: Vec<i32> = vec![];
            for coin in coins {
                let remaining = remaining - coin;
                if remaining < 0 {
                    continue;
                }

                let next_remaining_minimum = Self::coin_change_dp(remaining, coins, amount, memo);
                if next_remaining_minimum != -1 {
                    calc.push(1 + next_remaining_minimum);
                }
            }
            memo[remaining as usize] = calc.iter().min().unwrap_or(&-1).clone();
        }

        memo[remaining as usize]
    }
}

mod test {

    use super::*;

    #[test]
    fn test_coin_change_1() {
        let coins = vec![1, 2, 5];
        let ammount = 11;
        let result = Solution::coin_change(coins, ammount);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_coin_change_2() {
        let coins = vec![2];
        let ammount = 3;
        let result = Solution::coin_change(coins, ammount);
        assert_eq!(result, -1);
    }
}
