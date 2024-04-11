pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    if k == 0 {
        return;
    }
    let mut new = Vec::new();
    for i in 0..nums.len() {
        let j = (nums.len() + i - k) % nums.len();
        new.insert(i, nums[j]);
    }
    nums.copy_from_slice(&new[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    // nums = [1,2,3,4,5,6,7], k = 3
    #[test]
    fn test_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    // nums = [-1,-100,3,99], k = 2
    #[test]
    fn test_2() {
        let mut nums = vec![-1, -100, 3, 99];
        rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }
}