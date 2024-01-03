# 問題概要

## 難易度

Medium

## 問題内容

整数の配列 nums と整数 k が与えられたとき、最も頻繁に出現する k 個の要素を返してください。答えはどのような順序であっても構いません。
答えは一意であることが保証されています。

Example 1:

> Input: nums = [1,1,1,2,2,3], k = 2
> Output: [1,2]

## 考えたこと

ハッシュマップに各値の出現回数を集計する
> {1:3, 2:2, 3:1}

その後リストに格納する
> [0, 3, 2, 1]

最終的に逆順でループして必要な値だけ返す
> [1, 2]

## 感想

いわゆるバケットソートで回答してAcceptedだったが、RuntimeもMemoryも効率が悪かった。他のSolutionを確認したところより効率の良い方法が議論されていたのでメモ。

元のロジック
Runtime 7ms/Memory 3.5MB

ヒープを使うケース
Runtime 3ms/Memory 2.6MB

```rust
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for num in nums {
            let count = map.entry(num).or_insert(0);
            *count -= 1;
        }

        let mut heap = BinaryHeap::with_capacity(k as usize + 1);

        map.into_iter().for_each(|x| {
            heap.push((x.1, x.0));

            if heap.len() > k as usize {
                heap.pop();
            }
        });

        heap.into_iter().map(|x| x.1).collect::<Vec<i32>>()
    }
}
```

標準のカスタムソートを使うケース
Runtime 0ms/Memory2.6MB

```rust
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == (k as usize) {return nums}
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            let count = counts.entry(num).or_insert(0);
            *count += 1;
        }
        
        let mut freq: Vec<_> = counts.into_iter().collect();
        freq.sort_by(|a , b| b.1.cmp(&a.1));
        freq.into_iter().map(| (num, _ )| num).take(k as usize).collect()
    }
}
```
