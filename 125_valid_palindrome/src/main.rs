struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let lower_alphanumeric: Vec<char> = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        let len = lower_alphanumeric.len();
        for i in 0..len.div_ceil(2){
            if lower_alphanumeric[i] != lower_alphanumeric[len-1-i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();
    let result = Solution::is_palindrome(s);
    println!("{:?}", result);
}