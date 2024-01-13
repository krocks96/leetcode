struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push(c),
                _ => {
                    let popped = stack.pop();
                    if popped == None {
                        return false;
                    }
                    if !is_matched(popped.unwrap(), c) {
                        return false;
                    }
                }
            }
        }
        if stack.is_empty() { return false; }
        true
    }
}

fn is_matched(start: char, end: char) -> bool {
    if start == '(' && end == ')' { return true; }
    if start == '{' && end == '}' { return true; }
    if start == '[' && end == ']' { return true; }
    false
}

fn main() {
    let s = "()[]{}".to_string();
    let result = Solution::is_valid(s);
    println!("{:?}", result);
}