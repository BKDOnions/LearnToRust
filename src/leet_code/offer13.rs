use crate::algorithms::solution::Solution;

static mut COUNTER: i32 = 0;

impl Solution {
    pub unsafe fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut map: Vec<Vec<i32>> = vec![vec!(0; m as usize); n as usize];
        Solution::us_dfs(&mut map, 0, 0, k);
        return COUNTER;
    }
    unsafe fn us_dfs(map: &mut Vec<Vec<i32>>, m: i32, n: i32, k: i32) {
        if m >= map.len() as i32
            || m < 0
            || n >= map[0].len() as i32
            || n < 0
            || map[m as usize][n as usize] == 1
            || (m / 10 + m % 10 + n / 10 + n % 10) > k
        {
            return;
        } else {
            COUNTER += 1;
            map[m as usize][n as usize] = 1;
        }
        Solution::us_dfs(map, m + 1, n, k);
        Solution::us_dfs(map, m - 1, n, k);
        Solution::us_dfs(map, m, n - 1, k);
        Solution::us_dfs(map, m, n + 1, k);
    }
}
