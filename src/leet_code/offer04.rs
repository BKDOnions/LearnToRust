use crate::Solution;

impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty()
            || matrix.len() == 0
            || matrix.first().unwrap().is_empty()
            || matrix.first().unwrap().len() == 0
        {
            return false;
        }
        let len = matrix.first().unwrap().len();
        let mut x = 0usize;
        let mut y = 0usize;
        loop {
            if x == matrix.len() - 1 && y == len - 1 {
                return false;
            }
            if matrix[x][y] == target {
                return true;
            }
            if x < matrix.len() - 1 {
                x += 1;
            }
            if (x == matrix.len() - 1 || matrix[x][y] > target) && y < len - 1 {
                y += 1;
                x = 0;
            }
        }
    }
}

#[cfg(test)]
mod offer04 {
    use crate::Solution;

    #[test]
    fn offer04_test() {
        assert_eq!(
            Solution::find_number_in2_d_array(vec![vec![-1, 3]], 3),
            true
        )
    }
}
