struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // duplicate check
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for &num in &nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
        return false;
    }
}

fn main() {
    let nums = vec![1,2,3,1];
    println!("{:?}", nums);
    let has_duplicate = Solution::contains_duplicate(nums);
    println!("{}", has_duplicate);
}