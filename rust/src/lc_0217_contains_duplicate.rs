pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    
    let mut hs = HashSet::new();
    for num in nums {
        if hs.contains(&num) {
            return true
        }
        hs.insert(num);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // nums = [1,2,3,1]
    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    // nums = [1,2,3,4]
    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums), false);
    }

    // nums = [1,1,1,3,3,4,3,2,4,2]
    #[test]
    fn test_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contains_duplicate(nums), true);
    }
}