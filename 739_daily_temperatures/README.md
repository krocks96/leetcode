# 問題概要

## 難易度

Normal

## 問題内容

整数の配列 temperatures が毎日の気温を表しています。配列 answer を返してください。ただし、answer[i] は、i日目の後により暖かい気温を得るまでに待つ必要がある日数とします。もし、それが可能な未来の日が存在しない場合は、answer[i] を 0 としてください。

## 考えたこと

全探索するなら二重ループで最悪O(n^2)になりそう。ロジックは以下。
```rust
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answers: Vec<i32> = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            for j in i+1..temperatures.len() {
                if temperatures[i] >= temperatures[j] {
                    continue;
                }
                answers[i] = (j - i) as i32;
                break;
            }
        }
        return answers;
    }
}
```

が、ちょっと効率化の方法がわからない。

## 感想

時間をかけても分かりそうになかったのでヒントをもらって回答した。
ちなみに実行結果は以下の通り。
Runtime: 42ms / Memory: 5.25MB

このようなロジックを"Monotonic Stack"と呼ぶらしい。
https://qiita.com/awesam/items/458fb6330be4ab75c815

解きながらアルゴリズムを覚えていくより先に蟻本などをやった方がいいのだろうか…