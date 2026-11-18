impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .rev()
            .fold((0, 0), |(profit, max_price), &price| {
                (profit.max(max_price - price), max_price.max(price))
            })
            .0
    }
}
