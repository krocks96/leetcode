struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut stack: Vec<char> = Vec::new();
        Self::check(n, &mut result, &mut stack, 0, 0);
        return result;
    }

    fn check(n: i32, result: &mut Vec<String>, stack: &mut Vec<char>, open_n: i32, close_n: i32) {
        if open_n == n && close_n == n {
            let s: String = stack.iter().collect();
            result.push(s);
            return
        }
        if open_n < n {
            stack.push('(');
            Self::check(n, result, stack, open_n + 1, close_n);
            stack.pop();
        }
        if close_n < open_n {
            stack.push(')');
            Self::check(n, result, stack, open_n, close_n + 1);
            stack.pop();
        }
    }
}

fn main() {
    let n = 8;
    let result = Solution::generate_parenthesis(n);
    println!["{:?}", result];

}