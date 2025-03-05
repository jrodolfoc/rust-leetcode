struct Solution {
}

impl Solution {
    pub fn new() -> Self {
        Solution { }
    }

    pub fn jump(&self, nums: Vec<i32>) -> i32 {
        let mut jumps: i32 = 0;
        let mut biggest_jump: i32 = 0;
        let mut current_jump: i32 = 0;

        let mut index: i32 = 0;
        let nums_length: i32 = (nums.len() as i32) - 1;

        if nums_length < 1 {
            return 0;
        }

        for i in nums {
            biggest_jump = biggest_jump.max(index + i);
            if current_jump == index {
                jumps += 1;
                current_jump = biggest_jump;

            }

            index += 1;
            if index >= nums_length {
                break;
            }
        }

        jumps
    }
}

fn main() {
    let solution = Solution::new();
    
    let jumps1: i32 = solution.jump([2,3,1,1,4].to_vec());
    println!("Number of jumps: {jumps1}");

    let jumps2: i32 = solution.jump([2,3,0,1,4].to_vec());
    println!("Number of jumps: {jumps2}");

    let jumps3: i32 = solution.jump([1,2,3].to_vec());
    println!("Number of jumps: {jumps3}");
}
