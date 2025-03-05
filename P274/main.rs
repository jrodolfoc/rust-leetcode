struct Solution {
}

impl Solution {
    pub fn new() -> Self {
        Solution { }
    }

    pub fn h_index(&self, citations: Vec<i32>) -> i32 {
        let citations_length: i32 = citations.len() as i32;
        let mut h_index: i32 = citations_length;

        while h_index > 0 {
            let mut counter: i32 = 0;

            for i in &citations {
                if *i >= h_index {
                    counter += 1;
                }
            }

            if counter >= h_index {
                return h_index;
            }

            h_index -= 1;
        }

        0
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
