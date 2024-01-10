use std::cmp;
struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len()-1;
        while left < right {
            // 面積計算
            let left_h = height[left];
            let right_h = height[right];
            let area = (right - left) * cmp::min(left_h as usize, right_h as usize);
            println!("{:?}", area);
            // 最大値の置換
            if max_area < area { max_area = area; }
            // 添え字の更新
            if left_h < right_h {
                left += 1;
            } else {
                right -=1;
            }
        }
        return max_area as i32
    }
}

fn main() {
    let height = vec![1,8,6,2,5,4,8,3,7];
    let result = Solution::max_area(height);
    println!("{:?}", result);
}