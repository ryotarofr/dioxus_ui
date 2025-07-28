use dioxus::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

// Type aliases for complex function types
type InitFn = Rc<RefCell<dyn FnMut()>>;
type MaxPageFn = Rc<dyn Fn(usize) -> usize>;
type SetLimitFn = Rc<RefCell<dyn FnMut(usize)>>;

/// Parameters for the pagination hook
pub struct UsePaginationParams {
    /// Initial limit value
    pub init: usize,
    /// Optional disabled limit (overrides internal limit when set)
    pub disabled: Option<usize>,
    /// Current focused render index
    pub focused_render_index: Option<usize>,
}

/// Result type for the pagination hook
#[derive(Clone)]
pub struct UsePaginationResult {
    /// Initialize pagination to the init value
    pub init: InitFn,
    /// Current page number (0-based)
    pub page: usize,
    /// Minimum page number (always 0)
    pub min_page: usize,
    /// Function to calculate maximum page number
    pub max_page: MaxPageFn,
    /// Current limit (items per page)
    pub limit: usize,
    /// Current offset (starting index for current page)
    pub offset: usize,
    /// Whether pagination is disabled (limit is externally controlled)
    pub disabled: bool,
    /// Set the limit (items per page)
    pub set_limit: SetLimitFn,
}

/// Hook for managing pagination state in table view
/// 
/// This hook manages pagination logic including page calculation, limits, and offsets.
/// It can be disabled to use an external limit or use an internal state for the limit.
/// 
/// # Arguments
/// 
/// * `params` - Parameters containing initial limit, optional disabled limit, and focused render index
/// 
/// # Returns
/// 
/// UsePaginationResult containing pagination state and functions
/// 
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// 
/// #[component]
/// fn TableComponent() -> Element {
///     let pagination = use_pagination(UsePaginationParams {
///         init: 20,
///         disabled: None,
///         focused_render_index: Some(45),
///     });
///     
///     // Current page will be 2 (45 / 20 = 2)
///     let current_page = pagination.page;
///     
///     // Set new limit
///     pagination.set_limit.borrow_mut()(50);
///     
///     rsx! {
///         div { "Page {current_page + 1} of {pagination.max_page(100) + 1}" }
///     }
/// }
/// ```
pub fn use_pagination(params: UsePaginationParams) -> UsePaginationResult {
    // Internal state for limit - similar to useState in React
    let internal_limit = use_signal(move || params.init);
    
    // Determine the actual limit to use (disabled overrides internal)
    let limit = params.disabled.unwrap_or_else(|| *internal_limit.read());
    
    // Calculate current page based on focused render index
    let current_page = {
        let focused_index = params.focused_render_index.unwrap_or(0);
        if limit == 0 {
            0 // Avoid division by zero
        } else {
            focused_index / limit
        }
    };
    
    // Calculate offset (starting index for current page)
    let offset = current_page * limit;
    
    // Create init function to reset limit to initial value
    let init_fn = {
        let init_value = params.init;
        let mut internal_limit = internal_limit;
        Rc::new(RefCell::new(move || {
            internal_limit.set(init_value);
        }))
    };
    
    // Create setLimit function
    let set_limit_fn = {
        let mut internal_limit = internal_limit;
        Rc::new(RefCell::new(move |new_limit: usize| {
            internal_limit.set(new_limit);
        }))
    };
    
    // Create maxPage function
    let max_page_fn = {
        Rc::new(move |max_render_index: usize| -> usize {
            if limit == 0 {
                0 // Avoid division by zero
            } else {
                max_render_index / limit
            }
        })
    };
    
    UsePaginationResult {
        init: init_fn,
        page: current_page,
        min_page: 0,
        max_page: max_page_fn,
        limit,
        offset,
        disabled: params.disabled.is_some(),
        set_limit: set_limit_fn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_use_pagination_basic() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let pagination = use_pagination(UsePaginationParams {
                init: 10,
                disabled: None,
                focused_render_index: Some(25),
            });
            
            // Check basic calculations
            assert_eq!(pagination.limit, 10);
            assert_eq!(pagination.page, 2); // 25 / 10 = 2
            assert_eq!(pagination.offset, 20); // 2 * 10 = 20
            assert_eq!(pagination.min_page, 0);
            assert!(!pagination.disabled);
            
            // Test max_page function
            assert_eq!((pagination.max_page)(100), 10); // 100 / 10 = 10
            
            rsx! { div { "Pagination test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_pagination_disabled() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let pagination = use_pagination(UsePaginationParams {
                init: 10,
                disabled: Some(25), // Override with disabled limit
                focused_render_index: Some(50),
            });
            
            // Check that disabled limit is used
            assert_eq!(pagination.limit, 25);
            assert_eq!(pagination.page, 2); // 50 / 25 = 2
            assert_eq!(pagination.offset, 50); // 2 * 25 = 50
            assert!(pagination.disabled);
            
            rsx! { div { "Disabled pagination test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_pagination_zero_focused_index() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let pagination = use_pagination(UsePaginationParams {
                init: 10,
                disabled: None,
                focused_render_index: None, // Should default to 0
            });
            
            // Check defaults
            assert_eq!(pagination.page, 0);
            assert_eq!(pagination.offset, 0);
            
            rsx! { div { "Zero index test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_pagination_zero_limit() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let pagination = use_pagination(UsePaginationParams {
                init: 0, // Zero limit to test division by zero handling
                disabled: None,
                focused_render_index: Some(50),
            });
            
            // Check zero limit handling
            assert_eq!(pagination.limit, 0);
            assert_eq!(pagination.page, 0); // Should not crash
            assert_eq!(pagination.offset, 0);
            assert_eq!((pagination.max_page)(100), 0); // Should not crash
            
            rsx! { div { "Zero limit test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_pagination_set_limit() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let pagination = use_pagination(UsePaginationParams {
                init: 10,
                disabled: None,
                focused_render_index: Some(25),
            });
            
            // Test setting new limit
            pagination.set_limit.borrow_mut()(20);
            
            // Verify the function is callable
            // Note: In a real test, you'd need to re-render to see state changes
            assert_eq!(pagination.limit, 10); // Still the old value in this render
            
            rsx! { div { "Set limit test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_pagination_init() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let pagination = use_pagination(UsePaginationParams {
                init: 15,
                disabled: None,
                focused_render_index: Some(30),
            });
            
            // Test init function
            pagination.init.borrow_mut()();
            
            // Verify init function exists and is callable
            assert_eq!(pagination.limit, 15);
            
            rsx! { div { "Init test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_pagination_edge_cases() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            // Test with limit 1
            let pagination1 = use_pagination(UsePaginationParams {
                init: 1,
                disabled: None,
                focused_render_index: Some(5),
            });
            
            assert_eq!(pagination1.page, 5); // 5 / 1 = 5
            assert_eq!(pagination1.offset, 5); // 5 * 1 = 5
            
            // Test with very large numbers
            let pagination2 = use_pagination(UsePaginationParams {
                init: 100,
                disabled: None,
                focused_render_index: Some(999),
            });
            
            assert_eq!(pagination2.page, 9); // 999 / 100 = 9
            assert_eq!(pagination2.offset, 900); // 9 * 100 = 900
            
            rsx! { div { "Edge cases test" } }
        });
        
        dom.rebuild_to_vec();
    }
}
