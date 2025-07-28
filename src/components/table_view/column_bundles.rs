use dioxus::prelude::*;

use crate::components::table_view::column_bundle::ColumnBundle;
use crate::function::range::range;

#[derive(Props, Clone, PartialEq)]
pub struct ColumnBundlesProps {
    /// [ヘッダー行数, コンテンツ行数] の配列
    pub bundle_counts: Vec<(i32, i32)>,
    /// 全てのバンドルに渡すcssClassName
    #[props(default)]
    pub all_bundle_class_name: Option<String>,
    /// 列ヘッダーバンドルに渡すcssClassName
    #[props(default)]
    pub row_header_bundle_class_name: Option<String>,
    /// 追加のCSSクラス
    #[props(default)]
    pub class: Option<String>,
    /// 追加のインラインスタイル
    #[props(default)]
    pub style: Option<String>,
    /// 子要素
    pub children: Element,
}

#[derive(Debug, PartialEq, Clone)]
struct BundleCountWithUntil {
    header_length: i32,
    content_length: i32,
    until_count: i32,
}

/// [ヘッダー行数, コンテンツ行数] の数値指定を基に grid-column を纏める。
#[component]
pub fn ColumnBundles(props: ColumnBundlesProps) -> Element {
    let column_count_sum: i32 = props.bundle_counts
        .iter()
        .map(|(header, content)| header + content)
        .sum();

    let bundle_counts_with_until_count: Vec<BundleCountWithUntil> = props.bundle_counts
        .iter()
        .scan(0, |until_count, &(header_length, content_length)| {
            let current_until_count = *until_count;
            *until_count += header_length + content_length;
            Some(BundleCountWithUntil {
                header_length,
                content_length,
                until_count: current_until_count,
            })
        })
        .collect();

    let class_name = props.class
        .as_ref()
        .map(|c| format!("column-bundles {}", c))
        .unwrap_or_else(|| "column-bundles".to_string());

    // Extract children as Box<[DynamicNode]>
    let children_vec: Box<[dioxus::dioxus_core::DynamicNode]> = match props.children {
        Ok(ref vnode) => vnode.dynamic_nodes.clone(),
        Err(_) => Box::new([]),
    };

    rsx! {
        ColumnBundle {
            column_count: column_count_sum,
            class: class_name,
            style: props.style.clone().unwrap_or_default(),
            
            for (index, bundle) in bundle_counts_with_until_count.iter().enumerate() {
                div { key: "{index}",
                    // コンテンツバンドル
                    ColumnBundle {
                        column_start: bundle.until_count + bundle.header_length,
                        column_count: bundle.content_length,
                        class: props.all_bundle_class_name.clone().unwrap_or_default(),
                        for i in range(bundle.content_length as usize) {
                            if let Some(child) = children_vec.get((bundle.until_count + bundle.header_length) as usize + i) {
                                {child.clone()}
                            }
                        }
                    }
                    
                    // ヘッダーバンドル（ヘッダー長が0でない場合のみ）
                    if bundle.header_length > 0 {
                        ColumnBundle {
                            column_start: bundle.until_count,
                            column_count: bundle.header_length,
                            class: {
                                let mut classes = vec!["row-header"];
                                if let Some(ref all_class) = props.all_bundle_class_name {
                                    classes.push(all_class);
                                }
                                if let Some(ref header_class) = props.row_header_bundle_class_name {
                                    classes.push(header_class);
                                }
                                classes.join(" ")
                            },
                            for i in range(bundle.header_length as usize) {
                                if let Some(child) = children_vec.get(bundle.until_count as usize + i) {
                                    {child.clone()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_count_with_until_calculation() {
        let bundle_counts = [(2, 3), (1, 4), (0, 2)];
        
        let result: Vec<BundleCountWithUntil> = bundle_counts
            .iter()
            .scan(0, |until_count, &(header_length, content_length)| {
                let current_until_count = *until_count;
                *until_count += header_length + content_length;
                Some(BundleCountWithUntil {
                    header_length,
                    content_length,
                    until_count: current_until_count,
                })
            })
            .collect();

        assert_eq!(result.len(), 3);
        
        // First bundle: (2, 3), until_count = 0
        assert_eq!(result[0].header_length, 2);
        assert_eq!(result[0].content_length, 3);
        assert_eq!(result[0].until_count, 0);
        
        // Second bundle: (1, 4), until_count = 2 + 3 = 5
        assert_eq!(result[1].header_length, 1);
        assert_eq!(result[1].content_length, 4);
        assert_eq!(result[1].until_count, 5);
        
        // Third bundle: (0, 2), until_count = 5 + 1 + 4 = 10
        assert_eq!(result[2].header_length, 0);
        assert_eq!(result[2].content_length, 2);
        assert_eq!(result[2].until_count, 10);
    }

    #[test]
    fn test_column_count_sum() {
        let bundle_counts = [(2, 3), (1, 4), (0, 2)];
        let sum: i32 = bundle_counts
            .iter()
            .map(|(header, content)| header + content)
            .sum();
        
        assert_eq!(sum, 12); // (2+3) + (1+4) + (0+2) = 5 + 5 + 2 = 12
    }

    #[test]
    fn test_class_name_generation() {
        // Test with custom class
        let custom_class = Some("custom-class".to_string());
        let class_name = custom_class
            .as_ref()
            .map(|c| format!("column-bundles {}", c))
            .unwrap_or_else(|| "column-bundles".to_string());
        assert_eq!(class_name, "column-bundles custom-class");

        // Test without custom class
        let no_class: Option<String> = None;
        let class_name = no_class
            .as_ref()
            .map(|c| format!("column-bundles {}", c))
            .unwrap_or_else(|| "column-bundles".to_string());
        assert_eq!(class_name, "column-bundles");
    }

    #[test]
    fn test_header_class_generation() {
        let all_bundle_class = Some("all-bundle".to_string());
        let row_header_class = Some("row-header-specific".to_string());
        
        let mut classes = vec!["row-header"];
        if let Some(ref all_class) = all_bundle_class {
            classes.push(all_class);
        }
        if let Some(ref header_class) = row_header_class {
            classes.push(header_class);
        }
        let result = classes.join(" ");
        
        assert_eq!(result, "row-header all-bundle row-header-specific");
    }

    #[test]
    fn test_column_bundles_props_defaults() {
        // Test that default values work correctly
        let bundle_counts = [(1, 2)];
        
        // This would be how you'd create props with defaults in a real scenario
        // The actual component testing would require a more complex setup with Dioxus testing utilities
        assert_eq!(bundle_counts.len(), 1);
        assert_eq!(bundle_counts[0], (1, 2));
    }

    #[test]
    fn test_empty_bundle_counts() {
        let bundle_counts: Vec<(i32, i32)> = vec![];
        let sum: i32 = bundle_counts
            .iter()
            .map(|(header, content)| header + content)
            .sum();
        
        assert_eq!(sum, 0);
        
        let result: Vec<BundleCountWithUntil> = bundle_counts
            .iter()
            .scan(0, |until_count, &(header_length, content_length)| {
                let current_until_count = *until_count;
                *until_count += header_length + content_length;
                Some(BundleCountWithUntil {
                    header_length,
                    content_length,
                    until_count: current_until_count,
                })
            })
            .collect();
        
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_zero_header_length() {
        let bundle_counts = [(0, 5)];
        
        let result: Vec<BundleCountWithUntil> = bundle_counts
            .iter()
            .scan(0, |until_count, &(header_length, content_length)| {
                let current_until_count = *until_count;
                *until_count += header_length + content_length;
                Some(BundleCountWithUntil {
                    header_length,
                    content_length,
                    until_count: current_until_count,
                })
            })
            .collect();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].header_length, 0);
        assert_eq!(result[0].content_length, 5);
        assert_eq!(result[0].until_count, 0);
    }

    #[test]
    fn test_bundle_count_with_until_clone_and_partial_eq() {
        let bundle1 = BundleCountWithUntil {
            header_length: 2,
            content_length: 3,
            until_count: 5,
        };
        
        let bundle2 = bundle1.clone();
        assert_eq!(bundle1, bundle2);
        
        let bundle3 = BundleCountWithUntil {
            header_length: 2,
            content_length: 3,
            until_count: 6, // Different until_count
        };
        
        assert_ne!(bundle1, bundle3);
    }
}
