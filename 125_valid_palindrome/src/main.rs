struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered_chars: Vec<char> = s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        filtered_chars.iter().eq(filtered_chars.iter().rev())
    }
}

fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();
    let result = Solution::is_palindrome(s);
    println!("{:?}", result);
}