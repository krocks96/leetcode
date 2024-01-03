struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let mut answer: Vec<i32> = vec![1; nums_len];
        // 左側の計算
        for i in 1..nums_len {
            answer[i] = answer[i - 1] * nums[i - 1];
        }
        // 答えの計算
        let mut right = 1;
        for i in (0..nums_len).rev() {
            answer[i] *= right;
            right *= nums[i];
        }
        return answer;
    }
}

fn main() {
    let nums = vec![1,2,3,4];
    let result = Solution::product_except_self(nums);
    println!("{:?}", result);
}