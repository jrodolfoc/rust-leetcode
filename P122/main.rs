struct Solution {
}

impl Solution {
    pub fn new() -> Self {
        Solution { }
    }

    pub fn max_profit(&self, prices: Vec<i32>) -> i32 {
        let mut min_val:i32 = prices[0];
        let mut profit:i32 = 0;

        for price in prices {
            if min_val < price {
                profit += price - min_val;
            }

            min_val = price;
        } 

        profit
    }
}

fn main() {
    let solution = Solution::new();

    let profit:i32 = solution.max_profit([7,1,5,3,6,4].to_vec());
    println!("The Profit is {profit}");

    let profit2:i32 = solution.max_profit([1,2,3,4,5].to_vec());
    println!("The Profit is {profit2}");

    let profit3:i32 = solution.max_profit([7,6,4,3,1].to_vec());
    println!("The Profit is {profit3}");
}
