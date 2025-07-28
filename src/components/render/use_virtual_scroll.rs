use dioxus::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::function::range::{range, range_from};
use crate::function::signal::use_debounce::{use_debounce, DebounceProps};

/// Parameters for the virtual scroll hook
pub struct UseVirtualScrollParams {
    /// Default height for unrendered elements (in pixels)
    pub default_content_height_px: f64,
    /// Total number of elements
    pub content_length: usize,
    /// Number of elements to pre-render (defaults to 5)
    pub over_scan: Option<usize>,
}

/// Props to be passed to the VirtualScroll component
#[derive(Clone, Debug)]
pub struct VirtualScrollProps {
    /// Height before the visible area (in pixels)
    pub before_height_px: f64,
    /// Total height of all content (in pixels)
    pub max_height_px: f64,
}

/// Data item with its original index
#[derive(Clone, Debug)]
pub struct VirtualizedDataWithIndex<T> {
    /// The original data item
    pub get: T,
    /// The original index in the full data array
    pub data_index: usize,
}

// Type aliases for function types
type SetRenderCacheFn = Rc<dyn Fn(usize) -> Rc<RefCell<dyn FnMut(Option<String>)>>>;
type SetViewCacheFn = Rc<RefCell<dyn FnMut(Option<String>)>>;
type GetVirtualizedFn<T> = Rc<dyn Fn(&[T]) -> Vec<T>>;
type GetVirtualizedWithIndexFn<T> = Rc<dyn Fn(&[T]) -> Vec<VirtualizedDataWithIndex<T>>>;
type GetOffsetPxByIndexFn = Rc<dyn Fn(usize) -> f64>;

/// Result type for the virtual scroll hook
pub struct UseVirtualScrollResult<T> {
    /// Props to be passed to the VirtualScroll component
    pub props: VirtualScrollProps,
    /// Function to handle scroll events and update offset
    pub set_offset_on_scroll: Rc<RefCell<dyn FnMut(f64)>>,
    /// Get virtualized data slice
    pub get_virtualized: GetVirtualizedFn<T>,
    /// Get virtualized data with original indices
    pub get_virtualized_with_index: GetVirtualizedWithIndexFn<T>,
    /// Set render cache for a specific index
    pub set_render_cache: SetRenderCacheFn,
    /// Set view cache reference
    pub set_view_cache: SetViewCacheFn,
    /// Current view offset
    pub view_offset: usize,
    /// Current view limit
    pub view_limit: usize,
    /// Get offset pixels by index
    pub get_offset_px_by_index: GetOffsetPxByIndexFn,
}

/// Hook for virtual scrolling functionality
/// 
/// This hook provides virtual scrolling capability that only renders visible elements
/// within a scroll container, improving performance for large datasets.
/// 
/// # Arguments
/// 
/// * `params` - Configuration for virtual scrolling including default height and content length
/// 
/// # Returns
/// 
/// UseVirtualScrollResult containing all virtual scroll state and functions
/// 
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// 
/// #[component]
/// fn VirtualList(data: Vec<String>) -> Element {
///     let virtual_scroll = use_virtual_scroll(UseVirtualScrollParams {
///         default_content_height_px: 35.0,
///         content_length: data.len(),
///         over_scan: Some(5),
///     });
///     
///     let virtualized_data = (virtual_scroll.get_virtualized_with_index)(&data);
///     
///     rsx! {
///         div {
///             style: "overflow: auto; height: 400px; display: grid;",
///             onscroll: move |evt| {
///                 let scroll_top = evt.data.scroll_top() as f64;
///                 virtual_scroll.set_offset_on_scroll.borrow_mut()(scroll_top);
///             },
///             div {
///                 style: "height: {virtual_scroll.props.before_height_px}px;",
///             }
///             for item in virtualized_data {
///                 div {
///                     key: "{item.data_index}",
///                     style: "height: 35px;",
///                     "{item.get}"
///                 }
///             }
///             div {
///                 style: "height: {virtual_scroll.props.max_height_px - virtual_scroll.props.before_height_px}px;",
///             }
///         }
///     }
/// }
/// ```
pub fn use_virtual_scroll<T: Clone + 'static>(params: UseVirtualScrollParams) -> UseVirtualScrollResult<T> {
    let over_scan = params.over_scan.unwrap_or(5);
    let debounce = use_debounce(0);
    
    // Height map for tracking actual rendered element heights
    let height_map = use_signal(HashMap::<usize, f64>::new);
    
    // View container reference (element ID for tracking)
    let view_ref = use_signal(|| None::<String>);
    
    // Current scroll position
    let scrolled_px = use_signal(|| 0.0_f64);
    
    // Get sample content height (first available height or default)
    let sample_content_height_px = {
        let height_map_read = height_map.read();
        height_map_read.values()
            .find(|&&height| height > 0.0)
            .copied()
            .unwrap_or(params.default_content_height_px)
    };
    
    // Calculate view offset
    let raw_offsets = {
        let mut accumulator = AccumulatorState { px: 0.0, count: 0 };
        let height_map_read = height_map.read();
        let current_scroll = *scrolled_px.read();
        
        for index in range(params.content_length) {
            let height = height_map_read.get(&index).copied().unwrap_or(sample_content_height_px);
            if accumulator.px < current_scroll {
                accumulator.px += height;
                accumulator.count += 1;
            } else {
                break;
            }
        }
        accumulator
    };
    let view_offset = raw_offsets.count;
    let render_offset = raw_offsets.count.saturating_sub(over_scan);
    
    // Calculate view limit  
    let raw_limits = {
        let mut accumulator = AccumulatorState { px: 0.0, count: 0 };
        let height_map_read = height_map.read();
        // Assume a default view height if we don't have the actual container size
        let view_height = sample_content_height_px * 10.0; // Mock view height
        
        for index in range_from(params.content_length.saturating_sub(raw_offsets.count), raw_offsets.count) {
            let height = height_map_read.get(&index).copied().unwrap_or(sample_content_height_px);
            if accumulator.px < view_height {
                accumulator.px += height;
                accumulator.count += 1;
            } else {
                break;
            }
        }
        accumulator
    };
    let view_limit = raw_limits.count;
    let render_limit = raw_limits.count + over_scan;
    
    // Calculate heights
    let before_height_px = {
        let height_map_read = height_map.read();
        range(render_offset)
            .iter()
            .map(|&index| height_map_read.get(&index).copied().unwrap_or(sample_content_height_px))
            .sum()
    };
    
    let max_height_px = {
        let height_map_read = height_map.read();
        range(params.content_length)
            .iter()
            .map(|&index| height_map_read.get(&index).copied().unwrap_or(sample_content_height_px))
            .sum()
    };
    
    // Set offset on scroll function
    let set_offset_on_scroll_fn = {
        let debounce = debounce.clone();
        let mut scrolled_px = scrolled_px;
        
        Rc::new(RefCell::new(move |scroll_top: f64| {
            // Store the scroll value for immediate use
            scrolled_px.set(scroll_top);
            
            // Also trigger debounced callback for any additional processing
            debounce.borrow_mut()(Rc::new(move |_props: DebounceProps| {
                // Additional debounced processing could go here
            }));
        }))
    };
    
    // Get virtualized data function
    let get_virtualized_fn = {
        Rc::new(move |data: &[T]| -> Vec<T> {
            let end_index = (render_offset + render_limit).min(data.len());
            if render_offset < data.len() {
                data[render_offset..end_index].to_vec()
            } else {
                Vec::new()
            }
        })
    };
    
    // Get virtualized data with index function
    let get_virtualized_with_index_fn = {
        Rc::new(move |data: &[T]| -> Vec<VirtualizedDataWithIndex<T>> {
            let end_index = (render_offset + render_limit).min(data.len());
            if render_offset < data.len() {
                data[render_offset..end_index]
                    .iter()
                    .enumerate()
                    .map(|(index, item)| VirtualizedDataWithIndex {
                        get: item.clone(),
                        data_index: render_offset + index,
                    })
                    .collect()
            } else {
                Vec::new()
            }
        })
    };
    
    // Set render cache function
    let set_render_cache_fn = {
        let default_height = params.default_content_height_px;
        Rc::new(move |index: usize| -> Rc<RefCell<dyn FnMut(Option<String>)>> {
            let mut height_map = height_map;
            Rc::new(RefCell::new(move |element_id: Option<String>| {
                if let Some(_id) = element_id {
                    // In a real implementation, you would get the element's offsetHeight
                    // For now, we'll use a mock height calculation
                    let mock_height = default_height;
                    height_map.with_mut(|map| {
                        map.insert(index, mock_height);
                    });
                }
            }))
        })
    };
    
    // Set view cache function
    let set_view_cache_fn = {
        let mut view_ref = view_ref;
        Rc::new(RefCell::new(move |element_id: Option<String>| {
            view_ref.set(element_id);
        }))
    };
    
    // Get offset pixels by index function
    let get_offset_px_by_index_fn = {
        Rc::new(move |index: usize| -> f64 {
            let height_map_read = height_map.read();
            range(index.max(0))
                .iter()
                .map(|&i| height_map_read.get(&i).copied().unwrap_or(sample_content_height_px))
                .sum()
        })
    };
    
    UseVirtualScrollResult {
        props: VirtualScrollProps {
            before_height_px,
            max_height_px,
        },
        set_offset_on_scroll: set_offset_on_scroll_fn,
        get_virtualized: get_virtualized_fn,
        get_virtualized_with_index: get_virtualized_with_index_fn,
        set_render_cache: set_render_cache_fn,
        set_view_cache: set_view_cache_fn,
        view_offset,
        view_limit,
        get_offset_px_by_index: get_offset_px_by_index_fn,
    }
}

/// Helper struct for accumulating scroll calculations
#[derive(Debug, Clone)]
struct AccumulatorState {
    px: f64,
    count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_use_virtual_scroll_basic() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let virtual_scroll = use_virtual_scroll::<String>(UseVirtualScrollParams {
                default_content_height_px: 35.0,
                content_length: 100,
                over_scan: Some(5),
            });
            
            // Test that the virtual scroll structure is created
            assert!(virtual_scroll.props.before_height_px >= 0.0);
            assert!(virtual_scroll.props.max_height_px > 0.0);
            assert_eq!(virtual_scroll.view_offset, 0);
            
            rsx! { div { "Virtual scroll test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_virtual_scroll_props() {
        let props = VirtualScrollProps {
            before_height_px: 100.0,
            max_height_px: 3500.0,
        };
        
        assert_eq!(props.before_height_px, 100.0);
        assert_eq!(props.max_height_px, 3500.0);
    }
    
    #[test]
    fn test_virtualized_data_with_index() {
        let data = VirtualizedDataWithIndex {
            get: "test".to_string(),
            data_index: 5,
        };
        
        assert_eq!(data.get, "test");
        assert_eq!(data.data_index, 5);
        
        let cloned = data.clone();
        assert_eq!(cloned.get, "test");
        assert_eq!(cloned.data_index, 5);
    }
    
    #[test]
    fn test_get_virtualized_with_empty_data() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let virtual_scroll = use_virtual_scroll::<String>(UseVirtualScrollParams {
                default_content_height_px: 35.0,
                content_length: 0,
                over_scan: Some(5),
            });
            
            let data: Vec<String> = vec![];
            let virtualized = (virtual_scroll.get_virtualized)(&data);
            assert!(virtualized.is_empty());
            
            let virtualized_with_index = (virtual_scroll.get_virtualized_with_index)(&data);
            assert!(virtualized_with_index.is_empty());
            
            rsx! { div { "Empty data test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_get_virtualized_with_data() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let virtual_scroll = use_virtual_scroll::<String>(UseVirtualScrollParams {
                default_content_height_px: 35.0,
                content_length: 10,
                over_scan: Some(2),
            });
            
            let data: Vec<String> = (0..10).map(|i| format!("Item {}", i)).collect();
            let virtualized = (virtual_scroll.get_virtualized)(&data);
            
            // Should include over_scan items
            assert!(!virtualized.is_empty());
            assert!(virtualized.len() <= data.len());
            
            rsx! { div { "Data test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_accumulator_state() {
        let state = AccumulatorState { px: 100.0, count: 5 };
        assert_eq!(state.px, 100.0);
        assert_eq!(state.count, 5);
        
        let cloned = state.clone();
        assert_eq!(cloned.px, 100.0);
        assert_eq!(cloned.count, 5);
    }
}