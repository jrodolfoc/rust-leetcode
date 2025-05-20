struct Solution {
}

impl Solution {
    pub fn new() -> Self {
        Solution { }
    }

    pub fn reverse_words(&self, s: String) -> String {
        let mut words: Vec<&str> = s.split_whitespace().collect();
        words.reverse();
        words.join(" ")
    }
}

fn main() {
    let solution = Solution::new();

    let reverse1: String = solution.reverse_words("the sky is blue".to_string());
    println!("Reverse words: {reverse1}");

    let reverse2: String = solution.reverse_words("  hello world  ".to_string());
    println!("Reverse words: {reverse2}");

    let reverse3: String = solution.reverse_words("a good   example".to_string());
    println!("Reverse words: {reverse3}");
}

