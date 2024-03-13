pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    // nums = [3,2,2,3], val = 3
    #[test]
    fn test_1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(remove_element(&mut nums, val), 2);
        assert_eq!(nums, vec![2, 2]);
    }

    // nums = [0,1,2,2,3,0,4,2], val = 2
    #[test]
    fn test_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        assert_eq!(remove_element(&mut nums, val), 5);
        assert_eq!(nums, vec![0, 1, 3, 0, 4]);
    }
}