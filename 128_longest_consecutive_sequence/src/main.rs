use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // ハッシュセットに追加
        let num_set: HashSet<_> = nums.iter().cloned().collect();
        // 連続回数のチェック
        let mut max_seq = 0;
        for &num in &num_set {
            if !num_set.contains(&(num-1)) {
                let mut current_num = num;
                let mut current_seq = 1;
                // 連続する数を探す
                while num_set.contains(&(current_num+1)) {
                    current_num += 1;
                    current_seq += 1;
                }
                max_seq = max_seq.max(current_seq);
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