# 問題概要

## 難易度

Hard

## 問題内容

n個の非負の整数が与えられ、それぞれのバーの幅が1である標高マップを表しています。雨が降った後にそれがどれだけの水を溜めることができるかを計算してください。

## 考えたこと

まず幅が1なので高さの差が溜める水の量と同義
計算量を無視するのであれば、i番目のバーiに対して、同じ高さ以上のバーjが出るまで高さの差を集計すれば良い？
計算量は最悪O(n^2)になるので効率化できそうだが一旦これで実装してみる？
と思って実装してみたがうまくいかず。例えば、[4,2,3]の時に左から上記のロジックだと2になってしまう。

片方からだけだとNGなので左右から集計していく必要がありそう。

## 感想

１回愚直にやってみたがうまくいかないことに気付かず。。。
再提出でロジックを考え直してなんとかAC
Hardとはいえ1時間半くらいかかってしまった

Runtime: 2ms / Memory: 2.25MB

提出コードもロジックの重複や変数のスコープがおかしいため要修正&要復習