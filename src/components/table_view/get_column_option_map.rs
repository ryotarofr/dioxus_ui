use std::collections::HashMap;
use std::any::Any;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::function::get_mapped_object::get_mapped_object;
use crate::components::table_view::use_sort::SortOrder;

// Type aliases for complex function types
type ValueMapperFn<T> = Option<Rc<dyn Fn(&dyn Any, &RenderOptions<T>) -> String>>;
type AscSorterFn = Option<Rc<dyn Fn(&dyn Any, &dyn Any) -> Ordering>>;
type ValueMapperRequired<T> = Rc<dyn Fn(&dyn Any, &RenderOptions<T>) -> String>;
type AscSorterRequired = Rc<dyn Fn(&dyn Any, &dyn Any) -> Ordering>;

/// Rendering options context passed to value mappers
#[derive(Debug, Clone)]
pub struct RenderOptions<T> {
    pub data: T,
    pub id: String,
    pub is_focused: bool,
    pub is_selected: bool,
    pub data_index: usize,
    pub render_index: usize,
    pub local_index: usize,
}

/// Column configuration arguments for table display
#[derive(Clone)]
pub struct ColumnOptionArgs<T> {
    /// Header display content
    pub label: String,
    /// Cell content mapper function
    pub value_mapper: ValueMapperFn<T>,
    /// Whether this cell should be rendered as a header
    /// Default is `false`
    /// 
    /// Consecutive columns set to true become grouped row headers
    pub is_row_header: Option<bool>,
    /// Comparison function for sorting (ascending order)
    /// Return negative for no sort, positive for sort
    pub asc_sorter: AscSorterFn,
    /// Initial sort order on first render
    /// Default is `"none"` (no sort)
    pub init_sort_order: Option<SortOrder>,
    /// Whether sort order can be changed
    /// Default is `true`
    pub sort_order_is_changeable: Option<bool>,
    /// Whether to hide this column
    /// Default is `false`
    pub is_hidden: Option<bool>,
    /// Initial column width for parent's css[grid-template-columns]
    /// For example, `"max-content"` shrinks to maximum content width
    /// Default is `"minmax(max-content, 1fr)"`
    pub init_column_width: Option<String>,
    /// Text alignment setting
    /// Default is `"left"`, but specific types [numbers, dates] become `"right"`
    pub align: Option<String>,
    /// Whether to display total values
    /// Default is `false`
    pub total: Option<bool>,
}

/// Complete column option with all required fields
#[derive(Clone)]
pub struct ColumnOption<T> {
    pub key: String,
    pub label: String,
    pub value_mapper: ValueMapperRequired<T>,
    pub is_row_header: bool,
    pub asc_sorter: AscSorterRequired,
    pub init_sort_order: SortOrder,
    pub sort_order_is_changeable: bool,
    pub is_hidden: bool,
    pub init_column_width: String,
    pub align: String,
    pub total: bool,
}

/// Render map type for column configuration
/// 
/// Data structure definition for specifying rendering options.
/// 
/// Has several specification methods:
/// 
/// - Specify column name only
/// ```
/// { key: "Column1" }
/// ```
/// 
/// - Specify detailed settings for column
/// ```
/// {
///   key: {
///     label: "Column1",
///     value_mapper: |raw| format!("Value: {}", raw),
///     // ...
///   },
/// }
/// ```
pub type RenderMap<T> = HashMap<String, RenderMapValue<T>>;

#[derive(Clone)]
pub enum RenderMapValue<T> {
    Label(String),
    Options(ColumnOptionArgs<T>),
}

pub type ColumnOptionMap<T> = HashMap<String, ColumnOption<T>>;

/// Get label map from render map
pub fn get_label_map<T>(render_map: &RenderMap<T>) -> HashMap<String, String> {
    render_map
        .iter()
        .map(|(key, value)| {
            let label = match value {
                RenderMapValue::Label(label) => label.clone(),
                RenderMapValue::Options(options) => options.label.clone(),
            };
            (key.clone(), label)
        })
        .collect()
}

/// Convert RenderMap → ColumnOption
/// Convert from argument data format to internal logic data format
pub fn get_column_option_map<T>(render_map: &RenderMap<T>) -> ColumnOptionMap<T> {
    get_mapped_object(
        render_map.iter().map(|(k, v)| (k.clone(), v)).collect(),
        |(key, value), _index| get_column_option(key, value),
    )
}

/// Get default alignment based on raw value type
pub fn get_default_align(raw_value: &dyn Any) -> String {
    // Try to determine type by TypeId (limited but safe approach)
    use std::any::TypeId;
    
    let type_id = raw_value.type_id();
    
    if type_id == TypeId::of::<i32>() 
        || type_id == TypeId::of::<i64>() 
        || type_id == TypeId::of::<f32>() 
        || type_id == TypeId::of::<f64>() 
        || type_id == TypeId::of::<u32>() 
        || type_id == TypeId::of::<u64>() {
        "right".to_string()
    } else {
        "left".to_string()
    }
}

fn get_column_option<T>(key: &str, render_map_value: &RenderMapValue<T>) -> ColumnOption<T> {
    let default_option = default_column_option(key);
    
    match render_map_value {
        RenderMapValue::Label(label) => ColumnOption {
            label: label.clone(),
            ..default_option
        },
        RenderMapValue::Options(args) => ColumnOption {
            label: args.label.clone(),
            value_mapper: args.value_mapper
                .clone()
                .unwrap_or(default_option.value_mapper),
            is_row_header: args.is_row_header.unwrap_or(default_option.is_row_header),
            asc_sorter: args.asc_sorter
                .clone()
                .unwrap_or(default_option.asc_sorter),
            init_sort_order: args.init_sort_order
                .clone()
                .unwrap_or(default_option.init_sort_order),
            sort_order_is_changeable: args.sort_order_is_changeable
                .unwrap_or(default_option.sort_order_is_changeable),
            is_hidden: args.is_hidden.unwrap_or(default_option.is_hidden),
            init_column_width: args.init_column_width
                .clone()
                .unwrap_or(default_option.init_column_width),
            align: args.align
                .clone()
                .unwrap_or(default_option.align),
            total: args.total.unwrap_or(default_option.total),
            ..default_option
        },
    }
}

pub fn default_column_option<T>(key: &str) -> ColumnOption<T> {
    ColumnOption {
        key: key.to_string(),
        label: String::new(),
        value_mapper: Rc::new(|value, _options| {
            // Basic string conversion - in real implementation you'd want better type handling
            format!("{:?}", value)
        }),
        is_row_header: false,
        asc_sorter: Rc::new(|prev, next| {
            // Basic comparison - in real implementation you'd want better type handling
            format!("{:?}", prev).cmp(&format!("{:?}", next))
        }),
        init_sort_order: SortOrder::None,
        sort_order_is_changeable: !key.starts_with('_'),
        is_hidden: false,
        init_column_width: "minmax(max-content, 1fr)".to_string(),
        align: "left".to_string(),
        total: false,
    }
}

pub fn is_column_option_args_object<T>(value: &RenderMapValue<T>) -> bool {
    matches!(value, RenderMapValue::Options(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_label_map() {
        let mut render_map = HashMap::new();
        render_map.insert("key1".to_string(), RenderMapValue::<()>::Label("Label1".to_string()));
        render_map.insert("key2".to_string(), RenderMapValue::<()>::Options(ColumnOptionArgs {
            label: "Label2".to_string(),
            value_mapper: None,
            is_row_header: None,
            asc_sorter: None,
            init_sort_order: None,
            sort_order_is_changeable: None,
            is_hidden: None,
            init_column_width: None,
            align: None,
            total: None,
        }));

        let label_map = get_label_map(&render_map);
        
        assert_eq!(label_map.get("key1"), Some(&"Label1".to_string()));
        assert_eq!(label_map.get("key2"), Some(&"Label2".to_string()));
    }

    #[test]
    fn test_get_column_option_map() {
        let mut render_map = HashMap::new();
        render_map.insert("key1".to_string(), RenderMapValue::<()>::Label("Label1".to_string()));
        
        let column_option_map = get_column_option_map::<()>(&render_map);
        
        assert!(column_option_map.contains_key("key1"));
        let option = column_option_map.get("key1").unwrap();
        assert_eq!(option.label, "Label1");
        assert_eq!(option.key, "key1");
    }

    #[test]
    fn test_default_column_option() {
        let option = default_column_option::<()>("test_key");
        
        assert_eq!(option.key, "test_key");
        assert_eq!(option.label, "");
        assert!(!option.is_row_header);
        assert_eq!(option.init_sort_order, SortOrder::None);
        assert!(option.sort_order_is_changeable);
        assert!(!option.is_hidden);
        assert_eq!(option.init_column_width, "minmax(max-content, 1fr)");
        assert_eq!(option.align, "left");
        assert!(!option.total);
    }

    #[test]
    fn test_underscore_key_not_changeable() {
        let option = default_column_option::<()>("_internal_key");
        assert!(!option.sort_order_is_changeable);
        
        let option2 = default_column_option::<()>("normal_key");
        assert!(option2.sort_order_is_changeable);
    }

    #[test]
    fn test_get_default_align() {
        let num_val: &dyn Any = &42i32;
        let str_val: &dyn Any = &"hello";
        
        assert_eq!(get_default_align(num_val), "right");
        assert_eq!(get_default_align(str_val), "left");
    }
}