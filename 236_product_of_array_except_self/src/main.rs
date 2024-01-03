struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let mut left: Vec<i32> = vec![0; nums_len];
        let mut right: Vec<i32> = vec![0; nums_len];
        // 左右から計算
        for i in 0..nums_len {
            if i == 0 {
                left[i] = 1;
                right[i] = 1;
                continue;
            }
            left[i] = left[i-1]*nums[i-1];
            right[i] = right[i-1]*nums[nums_len-i];
        }
        // 答えの計算
        let mut answer: Vec<i32> = vec![0; nums_len];
        for i in 0..nums_len {
            answer[i] = left[i] * right[nums_len-1-i];
        }
        return answer;
    }
}

fn main() {
    let nums = vec![1,2,3,4];
    let result = Solution::product_except_self(nums);
    println!("{:?}", result);
}