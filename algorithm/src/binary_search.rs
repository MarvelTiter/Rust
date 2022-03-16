pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = nums.len() as i32 - 1;
    while start < end {
        let mid = start + (end - start) / 2;
        let value = nums[mid as usize];
        if value == target {
            return mid as i32;
        } else if value > target {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn binary_search_test() {
        let input = vec![-1,0,3,5,9,12];
        let target = 13;
        let result = binary_search(input, target);
        assert_eq!(result, -1);
    }
}
