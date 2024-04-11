pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;
    for i in 1..prices.len() {
        if prices[i] < min_price {
            min_price = prices[i];
        } else if prices[i] - min_price > max_profit {
            max_profit = prices[i] - min_price;
        }
    }
    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    // prices = [7,1,5,3,6,4]
    #[test]
    fn test_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    // prices = [7,6,4,3,1]
    #[test]
    fn test_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}