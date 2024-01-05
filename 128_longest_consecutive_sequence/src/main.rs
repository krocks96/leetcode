struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // イミュータブルなためclone
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        // 連続回数のチェック
        let mut max_seq = 0;
        let mut current_seq = 1;
        if sorted_nums.len() == 1 { return 1 }
        for i in 1..sorted_nums.len(){
            if sorted_nums[i] == sorted_nums[i-1] + 1 {
                current_seq += 1;
            } else if sorted_nums[i] > sorted_nums[i-1] + 1 {
                current_seq = 1;
            }
            if max_seq < current_seq {
                max_seq = current_seq;
            }
        }
       return max_seq;
    }
}

fn main() {
    let nums = vec![0,0];
    let result = Solution::longest_consecutive(nums);
    println!("{:?}", result);
}