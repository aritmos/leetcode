// best time to buy and sell stock
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut max_profit = 0;
        for price in prices {
            buy = i32::min(buy, price);
            max_profit = i32::max(max_profit, price - buy);
        }
        max_profit
    }
}
