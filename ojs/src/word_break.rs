use std::collections::HashMap;

use super::Solution;
/*
state:

recursion:

smallest

*/

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut memo: HashMap<&str, bool> = HashMap::new();
        Self::word_break_dp(&s, &word_dict, &mut memo)
    }

    fn word_break_dp<'a>(
        remaining: &'a str,
        word_dict: &Vec<String>,
        memo: &mut HashMap<&'a str, bool>,
    ) -> bool {
        if remaining.len() == 0 {
            return true;
        }

        if !memo.contains_key(remaining) {
            for word in word_dict {
                let next_remaining = remaining.strip_prefix(word);
                if let Some(s) = next_remaining {
                    let current = memo.get(remaining).unwrap_or(&false).clone();
                    let next = Self::word_break_dp(s, word_dict, memo);
                    memo.insert(remaining, current || next);
                }
            }
        }

        memo.get(remaining).unwrap_or(&false).clone()
    }
}

mod test {

    use super::*;

    #[test]
    fn test_word_break_1() {
        let word = String::from("applepenpenapple");
        let dict = vec![String::from("apple"), String::from("pen")];
        let result = Solution::word_break(word, dict);
        assert_eq!(result, true);
    }

    #[test]
    fn test_word_break_2() {
        let word = String::from("catsandog");
        let dict = vec![
            String::from("cats"),
            String::from("dog"),
            String::from("sand"),
            String::from("and"),
            String::from("cat"),
        ];
        let result = Solution::word_break(word, dict);
        assert_eq!(result, false);
    }

    #[test]
    fn test_word_break_3() {
        let word = String::from("abcd");
        let dict = vec![
            String::from("a"),
            String::from("abc"),
            String::from("b"),
            String::from("cd"),
        ];
        let result = Solution::word_break(word, dict);
        assert_eq!(result, true);
    }
}
