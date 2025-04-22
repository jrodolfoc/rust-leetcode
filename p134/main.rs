struct Solution {
}

impl Solution {
    pub fn new() -> Self {
        Solution { }
    }

    pub fn can_complete_circuit(&self, gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut tank:i32 = 0;
        let mut starting_station:i32 = 0;
        let mut tank_since_starting_station:i32 = -1;

        for i in 0..gas.len() {
            let delta = gas[i] - cost[i];
            tank += delta;

            if delta >= 0 && tank_since_starting_station < 0 {
                tank_since_starting_station = delta;
                starting_station = i as i32;
            } else {
                tank_since_starting_station += delta;
            }
        }

        if tank < 0 {
            -1
        } else {
            starting_station
        }
    }
}

fn main() {
    let solution = Solution::new();

    let output1:i32 = solution.can_complete_circuit([1,2,3,4,5].to_vec(), [3,4,5,1,2].to_vec());
    println!("The Output is {output1}\n");

    let output2:i32 = solution.can_complete_circuit([2,3,4].to_vec(), [3,4,3].to_vec());
    println!("The Output is {output2}\n");

    let output3:i32 = solution.can_complete_circuit([5,1,2,3,4].to_vec(), [4,4,1,5,1].to_vec());
    println!("The Output is {output3}\n");
}
//rustc main.rs