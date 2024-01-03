struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut counts: HashMap<i32, i32> = HashMap::new();
        let mut freq: Vec<Vec<i32>> = vec![vec![]; nums.len()+1];
        let mut result: Vec<i32> = vec![];
        // ハッシュマップで集計
        for num in nums {
            let count = counts.entry(num).or_insert(0);
            *count += 1;
        }
        // 頻度リストの生成
        for (k, v) in counts {
            freq[v as usize].push(k);
        }
        // 頻度が高いものから取得
        for i in (0..freq.len()).rev() {
            let vec = &freq[i];
            for &n in vec {
                result.push(n);
                if result.len() == k as usize {
                    return result;
                }
            }
        }
        return result;
    }
}

fn main() {
    let nums = vec![1,1,1,2,2,3];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    println!("{:?}", result);
}