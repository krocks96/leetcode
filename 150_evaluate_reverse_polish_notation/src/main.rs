struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut calculator: Vec<i32> = Vec::new();
        for token in tokens {
            let result = if matches!(token.as_str(), "+" | "-" | "*" | "/") {
                let right = calculator.pop().unwrap();
                let left = calculator.pop().unwrap();
                match token.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => 0, //本当は何もしたくないが書かないとエラーになる。unreachable!()で良いらしい。
                }
            } else {
                token.parse().unwrap()
            };
            calculator.push(result);
        }
        calculator.pop().unwrap()
    }
}

fn main() {
    let tokens:Vec<String> = vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
        .iter()
        .map(|&s|s.to_string())
        .collect();
    let result = Solution::eval_rpn(tokens);
    println!["{:?}", result];

}