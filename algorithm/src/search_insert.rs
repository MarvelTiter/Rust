pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
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
    end + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn search_insert_test() {
        let input = vec![1,3,5,6];
        let target = 0;
        let result = search_insert(input, target);
        assert_eq!(result, 0);
    }
}
