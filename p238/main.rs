struct Solution {
}

impl Solution {
    fn new() -> Self {
        Solution { }
    }

    fn product_except_self(&self, nums: Vec<i32>) -> Vec<i32> {
        let lenght = nums.len();
        let mut answer = vec![1; lenght];

        let mut prefix:i32 = 1;
        for i in 0..lenght {
            answer[i] = prefix;
            prefix *= nums[i];
        }

        let mut suffix:i32 = 1;
        for i in (0..lenght).rev() {
            answer[i] *= suffix;
            suffix *= nums[i];
        }

        answer
    }
}

fn main() {
    let solution = Solution::new();

    let product1:Vec<i32> = solution.product_except_self([1,2,3,4].to_vec());
    println!("The Product is {:?}", product1);

    let product2:Vec<i32> = solution.product_except_self([-1,1,0,-3,3].to_vec());
    println!("The Product is {:?}", product2);

    let product3:Vec<i32> = solution.product_except_self([-1,-1,2,-3,3].to_vec());
    println!("The Product is {:?}", product3);
}
