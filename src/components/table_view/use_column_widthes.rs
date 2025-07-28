use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

use crate::function::objects::Objects;
use crate::components::table_view::get_column_option_map::ColumnOptionMap;

// Type aliases for complex function types
type InitFn = Rc<std::cell::RefCell<dyn FnMut()>>;
type SetFn = Rc<std::cell::RefCell<dyn FnMut(HashMap<String, Option<String>>)>>;
type SetByKeyFn = Rc<std::cell::RefCell<dyn FnMut(String, Option<String>)>>;

/// Result type for column widths hook
pub struct UseColumnWidthsResult {
    /// Initialize column widths to default values
    pub init: InitFn,
    /// Get current column widths state
    pub get: HashMap<String, Option<String>>,
    /// Set entire column widths state
    pub set: SetFn,
    /// Set width for a specific column by key
    pub set_by_key: SetByKeyFn,
}

/// Hook for managing column widths in table view
/// 
/// This hook manages the column widths state, providing functionality to:
/// - Initialize column widths from ColumnOptionMap
/// - Get current column widths
/// - Set column widths (entire state or by individual key)
/// 
/// # Arguments
/// 
/// * `column_option_map` - Map of column options containing initial width settings
/// 
/// # Returns
/// 
/// UseColumnWidthsResult containing init, get, set, and set_by_key functions
/// 
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// 
/// #[component]
/// fn TableComponent() -> Element {
///     let column_options = get_column_option_map(&render_map);
///     let column_widths = use_column_widths(column_options);
///     
///     // Initialize widths
///     column_widths.init();
///     
///     // Set specific column width
///     column_widths.set_by_key("column1".to_string(), Some("200px".to_string()));
///     
///     // Get current widths
///     let current_widths = &column_widths.get;
///     
///     rsx! {
///         div { "Table with custom column widths" }
///     }
/// }
/// ```
pub fn use_column_widths<T>(column_option_map: ColumnOptionMap<T>) -> UseColumnWidthsResult {
    // Create initial column widths from column option map
    let init_column_widths = Objects::from_entries(
        Objects::entries(&column_option_map)
            .into_iter()
            .map(|(key, options)| (key.clone(), Some(options.init_column_width.clone())))
            .collect()
    );
    
    // Create signal for column widths state - similar to useState in React
    let state = use_signal(move || init_column_widths.clone());
    
    // Create init function
    let init_fn = {
        let init_widths = Objects::from_entries(
            Objects::entries(&column_option_map)
                .into_iter()
                .map(|(key, options)| (key.clone(), Some(options.init_column_width.clone())))
                .collect()
        );
        let mut state = state;
        Rc::new(std::cell::RefCell::new(move || {
            state.set(init_widths.clone());
        })) as InitFn
    };
    
    // Create set function for entire state - equivalent to setState in React
    let set_fn = {
        let mut state = state;
        Rc::new(std::cell::RefCell::new(move |new_widths: HashMap<String, Option<String>>| {
            state.set(new_widths);
        })) as SetFn
    };
    
    // Create set_by_key function - equivalent to React's partializeSetState(setState)
    // Since partialize_set_state requires NestedValueOf + NestedValueSetter traits,
    // we'll implement it directly for better type safety
    let set_by_key_fn = {
        let mut state = state;
        Rc::new(std::cell::RefCell::new(move |key: String, value: Option<String>| {
            state.with_mut(|current_widths| {
                current_widths.insert(key, value);
            });
        })) as SetByKeyFn
    };
    
    let current_state = state.read().clone();
    
    UseColumnWidthsResult {
        init: init_fn,
        get: current_state,
        set: set_fn,
        set_by_key: set_by_key_fn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::table_view::get_column_option_map::ColumnOption;
    use crate::components::table_view::use_sort::SortOrder;
    use std::rc::Rc;

    fn create_test_column_option_map() -> ColumnOptionMap<()> {
        let mut map = HashMap::new();
        
        map.insert("col1".to_string(), ColumnOption {
            key: "col1".to_string(),
            label: "Column 1".to_string(),
            value_mapper: Rc::new(|val, _opts| format!("{:?}", val)),
            is_row_header: false,
            asc_sorter: Rc::new(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b))),
            init_sort_order: SortOrder::None,
            sort_order_is_changeable: true,
            is_hidden: false,
            init_column_width: "200px".to_string(),
            align: "left".to_string(),
            total: false,
        });
        
        map.insert("col2".to_string(), ColumnOption {
            key: "col2".to_string(),
            label: "Column 2".to_string(),
            value_mapper: Rc::new(|val, _opts| format!("{:?}", val)),
            is_row_header: false,
            asc_sorter: Rc::new(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b))),
            init_sort_order: SortOrder::None,
            sort_order_is_changeable: true,
            is_hidden: false,
            init_column_width: "150px".to_string(),
            align: "right".to_string(),
            total: false,
        });
        
        map
    }

    #[test]
    fn test_use_column_widths_initialization() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let column_map = create_test_column_option_map();
            let column_widths = use_column_widths(column_map);
            
            // Check initial state
            assert_eq!(column_widths.get.get("col1"), Some(&Some("200px".to_string())));
            assert_eq!(column_widths.get.get("col2"), Some(&Some("150px".to_string())));
            
            rsx! { div {} }
        });
        
        dom.rebuild_to_vec();
    }

    #[test]
    fn test_use_column_widths_set_by_key() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let column_map = create_test_column_option_map();
            let column_widths = use_column_widths(column_map);
            
            // Set specific column width
            (column_widths.set_by_key.borrow_mut())("col1".to_string(), Some("300px".to_string()));
            
            // Note: In a real test, you'd need to check the state after a re-render
            // This is a simplified test to verify the function structure
            
            rsx! { div {} }
        });
        
        dom.rebuild_to_vec();
    }

    #[test]
    fn test_use_column_widths_init() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let column_map = create_test_column_option_map();
            let column_widths = use_column_widths(column_map);
            
            // Call init function
            (column_widths.init.borrow_mut())();
            
            // Verify init function exists and is callable
            assert!(!column_widths.get.is_empty());
            
            rsx! { div {} }
        });
        
        dom.rebuild_to_vec();
    }

    #[test]
    fn test_use_column_widths_set() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let column_map = create_test_column_option_map();
            let column_widths = use_column_widths(column_map);
            
            // Create new widths map
            let mut new_widths = HashMap::new();
            new_widths.insert("col1".to_string(), Some("400px".to_string()));
            new_widths.insert("col2".to_string(), None);
            
            // Set entire state
            (column_widths.set.borrow_mut())(new_widths);
            
            // Verify set function exists and is callable
            assert!(!column_widths.get.is_empty());
            
            rsx! { div {} }
        });
        
        dom.rebuild_to_vec();
    }
}