struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if set.contains(&diff) {
                return vec![i as i32, nums.iter().position(|&x| x == diff).unwrap() as i32];
            }
            set.insert(num);
        }
        panic!("Not found");
    }
}

fn main() {
    let two_sum = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", two_sum);
}