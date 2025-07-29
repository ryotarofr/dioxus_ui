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

## 2025_7_29
<!-- 保存時にフォーマッターが補完と自動成形してくれるようにしたい -->
<!-- Table の場合は、分割を自動で出す。改行時にRowも同じ長さの | を自動で出す -->
<!-- Text などの部分をクリック(若しくはショートカットキーなど)でスタイルを調整できるようにしたい -->
<!-- ※できるだけマウス操作なしで全部完結を目指す -->
Text      :
Headding1 :
Headding2 :
NumberList:
List      :
CodeBlock :
Table     : |  |  |  |
Row       : |  |  |  |
