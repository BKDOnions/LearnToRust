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
        let height = matrix.len();

        let mut index = 0;

        while if len > height {
            index < height
        } else {
            index < len
        } {
            let mut low = index;
            let mut high = height - 1;
            while low <= high {
                let mid = (low + high) >> 1;
                if matrix[mid][index] < target {
                    low = mid + 1;
                } else if matrix[mid][index] > target {
                    if mid > 0 {
                        high = mid - 1;
                    } else {
                        break;
                    }
                } else if matrix[mid][index] == target {
                    return true;
                } else {
                    break;
                }
            }
            let mut left = index;
            let mut right = len - 1;
            while left <= right {
                let mid = (left + right) >> 1;
                if matrix[index][mid] < target {
                    left = mid + 1;
                } else if matrix[index][mid] > target {
                    if mid > 0 {
                        right = mid - 1;
                    } else {
                        break;
                    }
                } else if matrix[index][mid] == target {
                    return true;
                } else {
                    break;
                }
            }
            index += 1;
        }

        return false;
    }
}

#[cfg(test)]
mod offer04 {
    use crate::Solution;

    #[test]
    fn offer04_test() {
        assert_eq!(Solution::find_number_in2_d_array(vec![vec![-5]], -10), true)
    }
}
