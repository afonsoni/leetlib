pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut j = 1;
    for i in 1..nums.len() {
        if j == 1 || nums[i] != nums[j - 2] {
            nums[j] = nums[i];
            j += 1;
        }
    }
    nums.resize(j, 0);
    j as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    // nums = [1,1,1,2,2,3]
    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![1, 1, 2, 2, 3]);
    }

    // nums = [0,0,1,1,1,1,2,3,3]
    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates(&mut nums), 7);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 3, 3]);
    }
}