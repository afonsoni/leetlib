pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    // nums = [1,1,2]
    #[test]
    fn test_1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        assert_eq!(nums, vec![1, 2]);
    }

    // nums = [0,0,1,1,1,2,2,3,3,4]
    #[test]
    fn test_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}