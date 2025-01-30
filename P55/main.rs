struct Solution {
}

impl Solution {
    pub fn new() -> Self {
        Solution { }
    }

    pub fn can_jump(&self, nums: Vec<i32>) -> bool {
        let mut energy = 0;
        let nums_length:usize = nums.len();

        if nums_length <= 1 {
            return true;
        }

        for i in 0..nums_length - 1 {
            energy -= 1;
            energy = energy.max(nums[i]);

            if energy < 1 {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    let solution = Solution::new();
    
    let can_jump1: bool = solution.can_jump([2,3,1,1,4].to_vec());
    println!("Can the person jump? {can_jump1}");

    let can_jump2: bool = solution.can_jump([3,2,1,0,4].to_vec());
    println!("Can the person jump? {can_jump2}");
    
    let can_jump3: bool = solution.can_jump([0,2,3].to_vec());
    println!("Can the person jump? {can_jump3}");
}
