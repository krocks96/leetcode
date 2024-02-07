struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = -1 as i32;
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            println!("{:?}", mid);
            if nums[mid] == target {
                result = mid as i32;
                break
            } else if nums[mid] < target {
                left = mid + 1;
                continue;
            } else if nums[mid] > target {
                if mid <= 0 { break; }
                right = mid - 1;
                continue;
            }
        }
        return result;
    }
}

fn main() {
    let target = 13 as i32;
    let nums = vec![-1,0,3,5,9,12];
    let result = Solution::search(nums, target);
    println!("{:?}", result);
}