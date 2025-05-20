struct Solution {
}

impl Solution {
    pub fn new() -> Self {
        Solution { }
    }

    pub fn int_to_roman(&self, num: i32) -> String {
        let values = [
            1000, 900, 500, 400,
            100,  90,  50,  40,
            10,   9,   5,   4,
            1,
        ];

        let symbols = [
            "M", "CM", "D", "CD",
            "C", "XC", "L", "XL",
            "X", "IX", "V", "IV",
            "I",
        ];

        let mut result = String::new();

        for i in 0..values.len() {
            while num >= values[i] {
                num -= values[i];
                result.push_str(symbols[i]);
            }
        }

        result
    }
}

fn main() {
    let solution = Solution::new();

    let roman1: String = solution.int_to_roman(3749);
    println!("Roman Number: {roman1}");

    let roman2: String = solution.int_to_roman(58);
    println!("Roman Number: {roman2}");

    let roman3: String = solution.int_to_roman(1994);
    println!("Roman Number: {roman3}");
}

