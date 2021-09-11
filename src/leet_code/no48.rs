use crate::algorithms::solution::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        let side = matrix.len() - 1;
        if side == 0 {
            return;
        }
        for i in 0..side / 2 + 1 {
            for j in i..side - i {
                let temp: i32 = matrix[i][j];
                matrix[i][j] = matrix[side - j][i];
                matrix[side - j][i] = matrix[side - i][side - j];
                matrix[side - i][side - j] = matrix[j][side - i];
                matrix[j][side - i] = temp;
            }
        }
    }
}
