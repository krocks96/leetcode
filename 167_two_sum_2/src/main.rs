struct Solution;
// use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // let mut map: HashMap<i32, i32> = HashMap::new();
        // let mut result: Vec<i32> = Vec::new();
        // for i in 0..numbers.len(){
        //     let diff = target - numbers[i];
        //     match map.get(&diff) {
        //         Some(&value) => {
        //             result.push(value);
        //             result.push((i+1) as i32);
        //             return result;
        //         },
        //         None => { map.insert(numbers[i], (i+1) as i32); },
        //     }
        // }
        // result
        // 2ポインタ法を使用する場合
        let (mut left, mut right) = (0, numbers.len()-1);
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == target {
                return vec![left as i32 + 1, right as i32 + 1];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        vec![]
    }
}

fn main() {
    let numbers = vec![2,7,11,15];
    let target = 9;
    let result = Solution::two_sum(numbers, target);
    println!("{:?}", result);
}