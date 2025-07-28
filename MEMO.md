- 🎉: 新機能 new feature
- 🐛: バグ修正 fix bug
- ♻️: リファクタリング refactor
- 📚: ドキュメント doc
- 🎨: デザインUI/UX degign
- 🐎: パフォーマンス performance
- 🔧: ツール tool
- 🚨: テスト test
- 💩: 非推奨追加 unko
- 🗑️: 削除 del
- 🚧: WIP
- 🚚: ファイルの移動 move
- 🔖: バージョンタグ tag
- you like this

## src/types/nested_key_of.rs について
ネストしたデータ構造内のすべての値へのパス（経路）を抽出するためのもの。

核となる仕組み

1. NestedKey enum: キーの種類を表現
  - String(String) - HashMap のキー
  - Number(usize) - Vec のインデックス
2. NestedKeyPath: 値への完全なパス（Vec<NestedKey>）
3. NestedKeyOf trait: 任意のデータ構造からすべての値へのパスを取得

  具体的な動作例

  // 例: ネストしたJSON風データ
  {
    "user": {
      "name": "Alice",
      "hobbies": ["reading", "coding"]
    }
  }

  この構造に対してget_nested_keys()を呼ぶと:
  [
    ["user", "name"],           // "Alice"への経路
    ["user", "hobbies", 0],     // "reading"への経路  
    ["user", "hobbies", 1]      // "coding"への経路
  ]

  実用的なメリット

  1. 動的フォーム生成

  // JSONスキーマから自動でフォームフィールドを生成
  let paths = data.get_nested_keys();
  for path in paths {
      create_input_field(path); // ["user", "name"] → input name="user.name"
  }

  2. バリデーションエラーのパス指定

  // エラー位置を正確に特定
  let error_path = vec![
      NestedKey::String("user".to_string()),
      NestedKey::String("email".to_string())
  ];
  show_error_at_path(error_path); // user.email フィールドにエラー表示

  3. テーブルの動的カラム生成

  // ネストしたデータを平坦なテーブルに展開
  let columns = data.get_nested_keys();
  // ["user.name", "user.hobbies.0", "user.hobbies.1"] のカラムを生成

  4. 設定ファイルエディタ

  // 設定の全項目を列挙して編集UIを生成
  let config_paths = config.get_nested_keys();
  for path in config_paths {
      render_config_field(path);
  }

  5. データ変更の追跡

  // 変更されたフィールドを正確に記録
  let changed_paths = get_changed_paths(old_data, new_data);
  // 部分更新APIに送信: PATCH /api/users {"user.email": "new@email.com"}

  テーブルビューでの活用

  プロジェクトのtable_viewコンポーネントでは、この機能により:
  - 複雑なオブジェクトを自動的にフラット化してテーブル表示
  - ネストした値でのソート機能
  - 動的な列の表示/非表示切り替え

  このように、構造化データを柔軟に扱うUIを構築する際に非常に有用な機能です。


## Setter
Setter<T>型でDioxusコンポーネントの状態更新関数を抽象化
