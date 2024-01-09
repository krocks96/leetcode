struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        let mut result = Vec::new();
        for i in 0..nums.len() - 2 {
            if i > 0 && sorted_nums[i] == sorted_nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = sorted_nums[i] + sorted_nums[left] + sorted_nums[right];
                if sum == 0 {
                    result.push(vec![sorted_nums[i] as i32, sorted_nums[left] as i32, sorted_nums[right] as i32]);
                    // 同じ数値を飛ばす
                    while left < right && sorted_nums[left] == sorted_nums[left + 1] {
                        left += 1;
                    }
                    while left < right && sorted_nums[right] == sorted_nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        result
    }
}

fn main() {
    let nums = vec![-1,0,1,2,-1,-4];
    let result = Solution::three_sum(nums);
    println!("{:?}", result);
}