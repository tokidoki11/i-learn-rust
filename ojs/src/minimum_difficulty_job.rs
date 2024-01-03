use super::Solution;

// state: jobIdx, currentDay
// recusive:
// similar to min_cost floor, but recurse dp until remaining day length = job to be done length
// jobs.len() - jobIdx = d - currentDay
// min(diffulty(jobIdx), dp(jobIdx+ 1, currentDay), dp(jobIdx+ 2, currentDay), ...)
// base condition:
// currentDay goes up, if currentDay == d, get remaining jobs

/*
The difficulty of a given day is the most difficult job that we did that day. Since the jobs have to be done in order,
if we are trying all the jobs we are allowed to do on that day (iterating through them),
then we can use a variable hardest
hardest to keep track of the difficulty of the hardest job done today.
If we choose to do jobs up to the jthjth job (inclusive), where i≤j<n - (d - day)i≤j<n - (d - day) (as derived above),
then that means on the next day, we start with the (j+1)th(j+1)th job. Therefore, our total difficulty is hardest + dp(j + 1, day + 1)hardest + dp(j + 1, day + 1).
This gives us our scariest recurrence relation so far:

dp(i, day) = min(hardest + dp(j + 1, day + 1)) for all i≤j<n - (d - day)i≤j<n - (d - day),
where hardest = max(jobDifficulty[k]) for all i≤k≤j.

*/

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        if job_difficulty.len() < d as usize {
            return -1;
        }
        let mut memo = vec![vec![0; job_difficulty.len() + 1]; (d + 1) as usize];
        Self::dp_min_dificulty(0, 1, &job_difficulty, &d, &mut memo)
    }

    fn dp_min_dificulty(
        i: usize,
        current_day: i32,
        jobs: &Vec<i32>,
        d: &i32,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if current_day == *d {
            return jobs[i..].iter().max().unwrap().clone();
        }

        if memo[i][current_day as usize] == 0 {
            let remaining_day = d - current_day;
            let max_taken_job_idx = (jobs.len() - remaining_day as usize);
            let mut hardest = 0;
            let mut best: i32 = std::i32::MAX;
            for j in i..max_taken_job_idx {
                hardest = hardest.max(jobs[j]);
                best = best
                    .min(hardest + Self::dp_min_dificulty(j + 1, current_day + 1, jobs, d, memo));
            }
            memo[i][current_day as usize] = best;
        }

        memo[i][current_day as usize]
    }
}

mod test {
    use super::*;

    #[test]
    fn test_minimum_difficulty_1() {
        let jobs = vec![6, 5, 4, 3, 2, 1];
        let result = Solution::min_difficulty(jobs, 2);
        assert_eq!(result, 7);
    }
    #[test]
    fn test_minimum_difficulty_2() {
        let jobs = vec![9, 9];
        let result = Solution::min_difficulty(jobs, 3);
        assert_eq!(result, -1);
    }
    #[test]
    fn test_minimum_difficulty_3() {
        let jobs = vec![1, 1, 1];
        let result = Solution::min_difficulty(jobs, 3);
        assert_eq!(result, 3);
    }
}
