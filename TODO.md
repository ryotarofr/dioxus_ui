## system design

TODO:
すべての UI のスタイルをインタラクティブに設定できる必要がある。
[kuma-ui](https://github.com/kuma-ui/kuma-ui)に近い設計思想を取り入れたい

↑ 仮想DOMにピッタリのツールになる

## TODO
ここからはテキストエディタに関する話

`<Box />` から作ろう

行毎のhtmlタグを入れる
このタグ自体が`<Box />`を返して挿入される？

まずは、`<Box />` が
Flex: justify-content(start,center,end)
を管理する仕組みを作る。

例えば、「ボタンs」「ボタンc」「ボタンe」でグリッドを動的変更できるようにする
