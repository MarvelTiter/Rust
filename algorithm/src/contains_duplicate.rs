use std::collections::HashMap;

pub fn contains_duplicate(nums: &mut Vec<i32>) -> bool {
    let mut map = HashMap::new();
    for v in nums {
        if map.contains_key(v) {
            return true;
        }
        map.insert(*v, 1);
    }
    return false;
}

#[cfg(test)]
mod contains_duplicate {
    use super::*;
    #[test]
    pub fn contains_duplicate_test() {
        let mut nums = vec![1,2,3,1];
        let result = contains_duplicate(&mut nums);
        assert_eq!(true, result);
    }
}
