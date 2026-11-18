impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max_price = prices.last().unwrap();

        for price in prices.iter().rev().skip(1) {
            result = result.max(max_price - price);
            max_price = max_price.max(price);
        }

        result
    }
}
