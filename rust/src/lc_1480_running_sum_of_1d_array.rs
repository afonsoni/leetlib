pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut out:Vec<i32> = Vec::new();
    let mut sum = 0;
    for n in nums {
        sum += n;
        out.push(sum);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // nums = [1,2,3,4]
    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(running_sum(nums), vec![1, 3, 6, 10]);
    }

    // nums = [1,1,1,1,1]
    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(running_sum(nums), vec![1, 2, 3, 4, 5]);
    }

    // nums = [3,1,2,10,1]
    #[test]
    fn test_3() {
        let nums = vec![3, 1, 2, 10, 1];
        assert_eq!(running_sum(nums), vec![3, 4, 6, 16, 17]);
    }
}