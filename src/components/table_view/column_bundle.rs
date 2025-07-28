use dioxus::prelude::*;

/// Usage:
/// 
/// ```rust
/// fn App() -> Element {
///     rsx! {
///         // スタイルを一度だけ読み込む
///         // TODO : もう少し良い方法はありそう
///         ColumnBundleStyles {}
///
///         // 通常のColumnBundle
///         ColumnBundle {
///             column_start: 2,
///             column_count: 4,
///             div { "通常のコンテンツ" }
///         }
///
///         // RowHeaderとして
///         ColumnBundle {
///             column_start: 0,
///             column_count: 2,
///             is_row_header: true,
///             div { "ヘッダーコンテンツ" }
///         }
///
///         // 非表示
///         ColumnBundle {
///             column_start: 3,
///             column_count: 1,
///             is_hidden: true,
///             div { "非表示コンテンツ" }
///         }
///     }
/// }
/// ```

#[derive(Props, Clone, PartialEq)]
pub struct ColumnBundleProps {
    /// 行開始位置
    #[props(default = 0)]
    column_start: i32,
    
    /// 内部行総数
    column_count: i32,
    
    /// 子要素
    #[props(default)]
    children: Element,
    
    /// 追加のクラス名
    #[props(default = "".to_string())]
    class: String,
    
    /// 追加のスタイル
    #[props(default = "".to_string())]
    style: String,
    
    /// RowHeaderとして表示するか
    #[props(default = false)]
    is_row_header: bool,
    
    /// 非表示にするか
    #[props(default = false)]
    is_hidden: bool,
    
    /// その他のHTML属性
    #[props(extends = div)]
    attributes: Vec<Attribute>,
}

/// Gridカラムを纏めるsubgrid親要素
#[component]
pub fn ColumnBundle(props: ColumnBundleProps) -> Element {
    let css_variables = format!(
        "--column-start: {}; --column-count: {};",
        props.column_start + 1,
        props.column_count
    );
    
    let combined_style = if props.style.is_empty() {
        css_variables
    } else {
        format!("{} {}", css_variables, props.style)
    };
    
    // クラス名の結合
    let mut classes = vec!["ColumnBundle"];
    
    if props.is_row_header {
        classes.push("RowHeader");
    }
    
    if props.is_hidden {
        classes.push("Hidden");
    }
    
    if !props.class.is_empty() {
        classes.push(&props.class);
    }
    
    let class_name = classes.join(" ");
    
    rsx! {
        div {
            class: "{class_name}",
            style: "{combined_style}",
            ..props.attributes,
            {props.children}
        }
    }
}

// スタイル定義
pub const COLUMN_BUNDLE_STYLES: &str = r#"
    .ColumnBundle {
        content: '';
        display: grid;
        grid-column: var(--column-start) / span var(--column-count);
        grid-template-columns: subgrid;
    }

    .RowHeader {
        content: '';
        grid-row: 1;
    }

    .Hidden {
        content: '';
        display: none;
    }
"#;

// スタイルを適用するコンポーネント
#[component]
pub fn ColumnBundleStyles() -> Element {
    rsx! {
        style { {COLUMN_BUNDLE_STYLES} }
    }
}
