struct Solution {
}

impl Solution {
    pub fn new() -> Self {
        Solution { }
    }

    pub fn h_index(&self, citations: Vec<i32>) -> i32 {
        let mut sorted_citations = citations.clone();
        sorted_citations.sort_unstable_by(|a, b| b.cmp(a));

        let mut h: i32 = 0;
        for (i, &citation) in sorted_citations.iter().enumerate() {
            if citation >= (i + 1) as i32 {
                h = (i + 1) as i32;
                continue;
            }

            break;
        }

        h
    }
}

fn main() {
    let solution = Solution::new();

    let h_index1:i32 = solution.h_index([3,0,6,1,5].to_vec());
    println!("The H-Index is {h_index1}");

    let h_index2:i32 = solution.h_index([1,3,1].to_vec());
    println!("The H-Index is {h_index2}");

    let h_index3:i32 = solution.h_index([7,6,4,3,1].to_vec());
    println!("The H-Index is {h_index3}");
}
