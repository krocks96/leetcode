struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answers: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, usize)> = Vec::new();
        for (i, t) in temperatures.iter().enumerate() {
            while stack.len() != 0 && *t > stack.last().unwrap().0 {
                let (stack_t, stack_i) = stack.pop().unwrap();
                answers[stack_i] = (i - stack_i) as i32;
            }
            stack.push((*t, i));
        }
        return answers;
    }
}

fn main() {
    let temperatures = vec![73,74,75,71,69,72,76,73];
    let result = Solution::daily_temperatures(temperatures);
    println!["{:?}", result];
}