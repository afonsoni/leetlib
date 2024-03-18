pub fn majority_element(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    let n = nums.len() as i32;

    // Count occurrences of each number
    for num in nums {
        let count = freq_map.entry(num).or_insert(0);
        *count += 1;
    }

    // Find the majority element
    for (num, freq) in freq_map.iter() {
        if *freq > n / 2 {
            return *num;
        }
    }

    -1 // No majority element found
}

#[cfg(test)]
mod tests {
    use super::*;

    // nums = [3,2,3]
    #[test]
    fn test1() {
        let nums = vec![3, 2, 3];
        assert_eq!(majority_element(nums), 3);
    }

    // nums = [2,2,1,1,1,2,2]
    #[test]
    fn test2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_element(nums), 2);
    }
}