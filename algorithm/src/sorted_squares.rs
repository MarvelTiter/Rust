pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut start = 0;
    let l = nums.len() as i32;
    let mut end = l - 1;
    let mut last = l - 1;
    let mut ret = vec![0; l as usize];
    while start <= end {
        let left = nums[start as usize] * nums[start as usize];
        let right = nums[end as usize] * nums[end as usize];
        if left > right {
            ret[last as usize] = left;
            last -= 1;
            start += 1;
        } else {
            ret[last as usize] = right;
            last -= 1;
            end -= 1;
        }
    }
    ret
}

#[cfg(test)]
mod sorted_square {
    use super::*;
    #[test]
    pub fn sorted_squares_test_01() {
        let nums = vec![-4, -1, 0, 3, 10];
        let result = sorted_squares(nums);
        assert_eq!(result, [0, 1, 9, 16, 100])
    }
}
