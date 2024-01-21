struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        // データ整形
        let mut sorted_position: Vec<(i32, i32)> = Vec::new();
        for i in 0..position.len() {
            sorted_position.push((position[i], speed[i]));
        }
        sorted_position.sort_by(|a, b| b.0.cmp(&a.0));
        // 集計
        let mut stack: Vec<f64> = Vec::new();
        for (p, s) in sorted_position {
            let t = (target - p) as f64 / s as f64;
            if stack.last().map_or(true, |&last| t > last) {
                stack.push(t);
            }
        }
        return stack.len() as i32;
    }
}

fn main() {
    let target = 10 as i32;
    let position = vec![8,3,7,4,6,5];
    let speed = vec![4,4,4,4,4,4];
    let result = Solution::car_fleet(target, position, speed);
    println!("{:?}", result);
}