# 問題概要

## 難易度

Medium

## 問題内容

整数の配列 nums が与えられたとき、i ≠ j、i ≠ k、そして j ≠ k を満たし、nums[i] + nums[j] + nums[k] == 0 となるすべての３つ組 [nums[i], nums[j], nums[k]] を返す。

解答セットには重複する組が含まれてはならないことに注意すること。

```
Example 1:

Input: nums = [-1,0,1,2,-1,-4]
Output: [[-1,-1,2],[-1,0,1]]
Explanation:
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
The distinct triplets are [-1,0,1] and [-1,-1,2].
Notice that the order of the output and the order of the triplets does not matter.
```

## 考えたこと

全探索するとnC3=n(n-1)(n-2)/6なのでO(n^3)となる。
どれか１つを固定した後、残りの要素に関して167と同様のロジックで解けるはず
時間複雑度はO(n^2)か？

## 感想

単純に実装したら重複する要素の排除を忘れておりWA
以下の処理を追加してAC
Runtime 31ms / Memory 4.06MB

```rust
// 略
// 同じ数値を飛ばす
while left < right && sorted_nums[left] == sorted_nums[left + 1] {
    left += 1;
}
while left < right && sorted_nums[right] == sorted_nums[right - 1] {
    right -= 1;
}
// 略
```

