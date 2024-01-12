struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;

        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    result += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    result += right_max - height[right];
                }
                right -= 1;
            }
        }

        result
    }
}

fn main() {
    let height = vec![4,2,0,3,2,5];
    let result = Solution::trap(height);
    println!("{:?}", result);
}