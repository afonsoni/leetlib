pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max_wealth: i32 = 0;
    for a in &accounts {
        let mut sum: i32 = 0;
        for i in a {
            sum += i;
        }
        if sum > max_wealth {
            max_wealth = sum;
        }
    }
    max_wealth
}

#[cfg(test)]
mod tests {
    use super::*;

    // [[1,2,3],[3,2,1]]
    #[test]
    fn test_1() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(maximum_wealth(accounts), 6);
    }

    // [[1,5],[7,3],[3,5]]
    #[test]
    fn test_2() {
        let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        assert_eq!(maximum_wealth(accounts), 10);
    }

    // [[2,8,7],[7,1,3],[1,9,5]]
    #[test]
    fn test_3() {
        let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        assert_eq!(maximum_wealth(accounts), 17);
    }
}