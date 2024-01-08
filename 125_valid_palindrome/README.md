# 問題概要

## 難易度

Easy

## 問題内容

フレーズが回文であるかどうかを判定する方法について説明します。すべての大文字を小文字に変換し、非英数字の文字を取り除いた後、そのフレーズが前から読んでも後ろから読んでも同じであれば、それは回文です。英数字には文字と数字が含まれます。

## 考えたこと

文字列の長さの半分の回数ループして、s[i]とs[n-1-i]が一致するかチェックしていけばいいのでは？

## 感想

Runtime 3ms / Memory 2.54MB

rustでの英数字以外を取り除く処理がわからなかったので調べた結果、is_alphanumeric()なんて便利なロジックがあった。
処理自体は通ったが時間、メモリともに効率が良くないので要修正。

<hr/>
以下改善ポイント

* 新たにVec<char>を作成しているが、イテレータを直接操作する方法があった

* そもそもto_lowercase()よりto_ascii_lowercase()の方が良さそう

https://qiita.com/dalance/items/5816618a308e0058136b

> Rustの文字列型はエンコードがUTF-8と決まっているので、Unicodeに関連したメソッドがいくつかあり、少し珍しいかもしれません。例えば、to_ascii_lowercase はASCIIの範囲内のみの小文字化、to_lowercaseはUnicode定義に従った小文字化です。
> ちなみにこのΣですが、単語の末尾に登場するときだけ、小文字化するとσではなくςになるらしいです。to_lowercaseはそのあたりもケアしてくれますが、その分処理は重いはずなので、ASCIIの範囲内だけ小文字化すればいいならto_ascii_lowercaseの方が高速だということでしょう。