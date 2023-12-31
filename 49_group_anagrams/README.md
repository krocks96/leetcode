# 問題概要

## 難易度

Normal

## 問題内容

文字列の配列 strs が与えられたとき、アナグラムをまとめてください。答えはどの順序であっても構いません。
アナグラムとは、異なる単語やフレーズの文字を並び替えて形成される単語やフレーズのことで、通常は元の文字を一度ずつ正確に使用します。

Example 1:

> Input: strs = ["eat","tea","tan","ate","nat","bat"]
> Output: [ ["bat"],["nat","tan"],["ate","eat","tea"] ]

## 考えたこと

与えられた文字列をソートしたものをキーとして管理すれば良さそう
"aet" -> ["eat", "tea", "ate"]のようなグループを作るイメージ

文字列をそのままソートできるのかは不明
→バイト配列にして変換するのが良さそう


## 感想

ロジック自体は検討通りだったが、`map.values()`を使おうとして効率が良くない書き方である。
各値のクローンが生成されるのでメモリ消費量が増える。
```rust
    let mut result: Vec<Vec<String>> = Vec::new();
    for value in map.values(){
        result.push(value.to_vec());
    }
    return result
```

以下の方が良い
```rust
    map.into_iter().map(|(_, v)| v).collect()
```
