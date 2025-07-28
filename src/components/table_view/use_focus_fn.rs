use dioxus::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::function::signal::use_effect_event::use_effect_event;
use crate::types::setter::{SetStateAction, SetterUtils};
use crate::components::table_view::use_focus::UseFocusResult;

// Type aliases for complex function types
pub type GetIndexFromIdFn = Rc<dyn Fn(Option<String>) -> Option<usize>>;
pub type GetIdFromIndexFn = Rc<dyn Fn(Option<usize>) -> Option<String>>;
pub type GetScrollOffsetPxFn = Rc<dyn Fn(usize) -> Option<f64>>;
pub type FocusByIdFn = Rc<RefCell<dyn FnMut(SetStateAction<Option<String>>, Option<FocusByIdOptions>)>>;
pub type FocusByRenderIndexFn = Rc<RefCell<dyn FnMut(SetStateAction<usize>, Option<FocusByRenderIndexOptions>)>>;
pub type FocusByLocalIndexFn = Rc<RefCell<dyn FnMut(SetStateAction<Option<usize>>, Option<FocusByLocalIndexOptions>)>>;
pub type SetScrollRefFn = Rc<RefCell<dyn FnMut(Option<String>)>>;

/// Options for focusing by ID
#[derive(Clone)]
pub struct FocusByIdOptions {
    /// If true, don't perform scrolling. Default is false
    pub without_scroll: Option<bool>,
    /// If true, also perform selection. Default is !select_many
    pub with_select: Option<SetStateAction<bool>>,
}

impl Default for FocusByIdOptions {
    fn default() -> Self {
        Self {
            without_scroll: Some(false),
            with_select: None,
        }
    }
}

/// Options for focusing by render index
#[derive(Clone)]
pub struct FocusByRenderIndexOptions {
    /// If true, store renderIndex even if target doesn't exist and apply after rendering update
    pub fallback: Option<bool>,
    /// If true, clamp to data range. Default is true
    pub molded: Option<bool>,
    /// Base options
    pub base: Option<FocusByIdOptions>,
}

impl Default for FocusByRenderIndexOptions {
    fn default() -> Self {
        Self {
            fallback: Some(false),
            molded: None,
            base: None,
        }
    }
}

/// Options for focusing by local index
#[derive(Clone)]
pub struct FocusByLocalIndexOptions {
    /// If true, clamp to data range. Default is true
    pub molded: Option<bool>,
    /// If true, handle overflow by flooring. Default is false
    pub floor_overflow: Option<bool>,
    /// Base options
    pub base: Option<FocusByIdOptions>,
}

impl Default for FocusByLocalIndexOptions {
    fn default() -> Self {
        Self {
            molded: Some(true),
            floor_overflow: Some(false),
            base: None,
        }
    }
}

/// Parameters for the useFocusFn hook
// Type alias for select_by_id function type
type SelectByIdFn = Rc<dyn Fn(String) -> Rc<RefCell<dyn FnMut(SetStateAction<bool>) -> crate::components::table_view::use_select::SelectResult>>>;

pub struct UseFocusFnParams {
    pub focus: UseFocusResult,
    pub select_by_id: SelectByIdFn,
    pub get_render_index_from_id: GetIndexFromIdFn,
    pub get_id_from_render_index: GetIdFromIndexFn,
    pub get_local_index_from_id: GetIndexFromIdFn,
    pub get_id_from_local_index: GetIdFromIndexFn,
    pub get_scroll_offset_px: GetScrollOffsetPxFn,
    pub max_render_index: usize,
    pub max_local_index: usize,
    pub select_many: bool,
}

/// Result type for the useFocusFn hook
pub struct UseFocusFnResult {
    /// Current focused ID with fallback
    pub id: Option<String>,
    /// Set focus by ID
    pub set_by_id: FocusByIdFn,
    /// Set focus by render index
    pub set_by_render_index: FocusByRenderIndexFn,
    /// Set focus by local index
    pub set_by_local_index: FocusByLocalIndexFn,
    /// Set scroll container reference
    pub set_scroll_ref: SetScrollRefFn,
}

/// Hook for managing focus functionality with scrolling and selection integration
/// 
/// This hook provides high-level focus management that integrates with scrolling and selection.
/// It handles fallback indices, scrolling behavior, and selection coordination.
/// 
/// # Arguments
/// 
/// * `params` - Parameters containing focus state, selection functions, index mapping functions, and configuration
/// 
/// # Returns
/// 
/// UseFocusFnResult containing focus management functions
/// 
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// 
/// #[component]
/// fn TableComponent() -> Element {
///     let focus = use_focus(None);
///     let select = use_select(/* ... */);
///     
///     let focus_fn = use_focus_fn(UseFocusFnParams {
///         focus,
///         select_by_id: select.set_by_id,
///         // ... other params
///     });
///     
///     rsx! {
///         div { "Table with advanced focus management" }
///     }
/// }
/// ```
pub fn use_focus_fn(params: UseFocusFnParams) -> UseFocusFnResult {
    // State management for scroll and fallback
    let mut scroll_reserved = use_signal(|| false);
    let default_fallback_render_index = 0;
    let fallback_render_index = use_signal(move || default_fallback_render_index);
    
    let set_fallback_render_index = {
        let mut fallback_render_index = fallback_render_index;
        Rc::new(RefCell::new(move |render_index: Option<usize>| {
            fallback_render_index.set(render_index.unwrap_or(default_fallback_render_index));
        }))
    };
    
    let get_fallbacked_render_index = {
        move |render_index: Option<usize>| -> usize {
            render_index.unwrap_or_else(|| *fallback_render_index.read())
        }
    };
    
    let get_fallbacked_id = {
        let get_id_from_render_index = params.get_id_from_render_index.clone();
        move |id: Option<String>| -> Option<String> {
            id.or_else(|| (get_id_from_render_index)(Some(*fallback_render_index.read())))
        }
    };
    
    // Current render index
    let render_index = (params.get_render_index_from_id)(params.focus.id.clone());
    
    // Scroll function
    let scroll_to = {
        let get_local_index_from_id = params.get_local_index_from_id.clone();
        let get_scroll_offset_px = params.get_scroll_offset_px.clone();
        move |next_id: Option<String>| {
            // In a real implementation, this would use DOM APIs to scroll
            // For now, we'll just track the scroll offset calculation
            if let Some(id) = next_id {
                if let Some(local_index) = (get_local_index_from_id)(Some(id)) {
                    let _offset_px = (get_scroll_offset_px)(local_index);
                    // parent?.scrollTo({ top: offsetPx });
                }
            }
        }
    };
    
    // Effect event for scroll firing when render position changes
    let fire_scroll = {
        let focus_id = params.focus.id.clone();
        use_effect_event(move |()| {
            scroll_to(focus_id.clone());
        })
    };
    
    // Effect to trigger scroll when render index changes
    use_effect(move || {
        if render_index.is_some() && *scroll_reserved.read() {
            fire_scroll.borrow_mut()(());
            scroll_reserved.set(false);
        }
    });
    
    // Set scroll ref function
    let set_scroll_ref_fn = {
        let mut scroll_reserved = scroll_reserved;
        let focus_set_scroll_ref = params.focus.set_scroll_ref.clone();
        Rc::new(RefCell::new(move |ref_id: Option<String>| {
            if let Some(_ref_id) = ref_id {
                scroll_reserved.set(true);
                focus_set_scroll_ref.borrow_mut()(Some(_ref_id));
            }
        }))
    };
    
    // Focus by ID function
    let focus_by_id_fn = {
        let mut scroll_reserved = scroll_reserved;
        let focus_set_id = params.focus.set_id.clone();
        let select_by_id = params.select_by_id.clone();
        let select_many = params.select_many;
        let get_fallbacked_id_closure = get_fallbacked_id.clone();
        let focus_id_for_closure = params.focus.id.clone(); // Clone early to avoid move issues
        
        Rc::new(RefCell::new(move |set_state_action: SetStateAction<Option<String>>, local_options: Option<FocusByIdOptions>| {
            let options = local_options.unwrap_or_default();
            let without_scroll = options.without_scroll.unwrap_or(false);
            let with_select = options.with_select.unwrap_or(SetStateAction::Value(!select_many));
            
            // Calculate the new focus ID
            let prev_id = get_fallbacked_id_closure(focus_id_for_closure.clone());
            let next_id = SetterUtils::to_value(set_state_action.clone(), prev_id.clone());
            
            if !without_scroll {
                scroll_reserved.set(true);
            }
            
            if !matches!(with_select, SetStateAction::Value(false)) {
                if let Some(ref next_id_str) = next_id {
                    // Handle selection change
                    let select_result = (select_by_id)(next_id_str.clone()).borrow_mut()(SetStateAction::Function(Rc::new({
                        let next_id_clone = next_id.clone();
                        let focus_set_id_clone = focus_set_id.clone();
                        let with_select_clone = with_select.clone();
                        move |prev_selected: bool| {
                            // Set focus ID in callback to handle delayed execution
                            focus_set_id_clone.borrow_mut()(next_id_clone.clone());
                            SetterUtils::to_value(with_select_clone.clone(), prev_selected)
                        }
                    })));
                    
                    if select_result.default_prevented {
                        return; // Don't update focus if selection was prevented
                    }
                }
            }
            
            // Set the focus ID directly
            focus_set_id.borrow_mut()(next_id);
        }))
    };
    
    // Focus by render index function
    let focus_by_render_index_fn = {
        let focus_by_id = focus_by_id_fn.clone();
        let get_fallbacked_render_index_closure = get_fallbacked_render_index;
        let get_render_index_from_id = params.get_render_index_from_id.clone();
        let get_id_from_render_index = params.get_id_from_render_index.clone();
        let max_render_index = params.max_render_index;
        let set_fallback_render_index_closure = set_fallback_render_index.clone();
        
        Rc::new(RefCell::new(move |set_state_action: SetStateAction<usize>, local_options: Option<FocusByRenderIndexOptions>| {
            let options = local_options.unwrap_or_default();
            let fallback = options.fallback.unwrap_or(false);
            let molded = options.molded.unwrap_or(!fallback);
            
            let get_render_index_from_id_clone = get_render_index_from_id.clone();
            let get_id_from_render_index_clone = get_id_from_render_index.clone();
            let set_fallback_render_index_clone = set_fallback_render_index_closure.clone();
            let get_fallbacked_render_index_clone = get_fallbacked_render_index_closure;
            
            focus_by_id.borrow_mut()(SetStateAction::Function(Rc::new(move |prev_id: Option<String>| {
                let prev_render_index = get_fallbacked_render_index_clone((get_render_index_from_id_clone)(prev_id));
                let next_render_index = {
                    let raw = SetterUtils::to_value(set_state_action.clone(), prev_render_index);
                    if molded {
                        Some(get_molded_index(0, raw, max_render_index))
                    } else {
                        Some(raw)
                    }
                };
                
                if fallback {
                    set_fallback_render_index_clone.borrow_mut()(next_render_index);
                }
                
                (get_id_from_render_index_clone)(next_render_index)
            })), options.base);
        }))
    };
    
    // Focus by local index function
    let focus_by_local_index_fn = {
        let focus_by_id = focus_by_id_fn.clone();
        let get_local_index_from_id = params.get_local_index_from_id.clone();
        let get_id_from_local_index = params.get_id_from_local_index.clone();
        let max_local_index = params.max_local_index;
        
        Rc::new(RefCell::new(move |set_state_action: SetStateAction<Option<usize>>, local_options: Option<FocusByLocalIndexOptions>| {
            let options = local_options.unwrap_or_default();
            let molded = options.molded.unwrap_or(true);
            let floor_overflow = options.floor_overflow.unwrap_or(false);
            
            let get_local_index_from_id_clone = get_local_index_from_id.clone();
            let get_id_from_local_index_clone = get_id_from_local_index.clone();
            
            focus_by_id.borrow_mut()(SetStateAction::Function(Rc::new(move |prev_id: Option<String>| {
                let prev_local_index = (get_local_index_from_id_clone)(prev_id);
                let next_local_index = {
                    if let Some(raw) = SetterUtils::to_value(set_state_action.clone(), prev_local_index) {
                        if floor_overflow {
                            Some(get_overflow_floored_index(raw, max_local_index))
                        } else if molded {
                            Some(get_molded_index(0, raw, max_local_index))
                        } else {
                            Some(raw)
                        }
                    } else {
                        None
                    }
                };
                
                (get_id_from_local_index_clone)(next_local_index)
            })), options.base);
        }))
    };
    
    // Get current ID with fallback
    let current_id = get_fallbacked_id(params.focus.id);
    
    UseFocusFnResult {
        id: current_id,
        set_by_id: focus_by_id_fn,
        set_by_render_index: focus_by_render_index_fn,
        set_by_local_index: focus_by_local_index_fn,
        set_scroll_ref: set_scroll_ref_fn,
    }
}

/// Clamp index to valid range
fn get_molded_index(min: usize, raw: usize, max_plus_one: usize) -> usize {
    if max_plus_one == 0 {
        min
    } else {
        min.max(raw.min(max_plus_one - 1))
    }
}

/// Handle overflow by flooring to valid range
fn get_overflow_floored_index(raw: usize, max_plus_one: usize) -> usize {
    if max_plus_one <= raw {
        0
    } else {
        raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::table_view::use_focus::{use_focus, FocusInit};
    use crate::components::table_view::use_select::SelectResult;
    
    // Type alias for the mock select_by_id function type
    type MockSelectByIdFn = Rc<dyn Fn(String) -> Rc<RefCell<dyn FnMut(SetStateAction<bool>) -> SelectResult>>>;

    fn create_test_params() -> UseFocusFnParams {
        let focus = use_focus(Some(FocusInit::Value("test-1".to_string())));
        
        // Mock select function with explicit type annotation
        let select_by_id: MockSelectByIdFn = Rc::new(move |_id: String| {
            Rc::new(RefCell::new(move |_action: SetStateAction<bool>| SelectResult {
                default_prevented: false,
            })) as Rc<RefCell<dyn FnMut(SetStateAction<bool>) -> SelectResult>>
        });
        
        // Mock index mapping functions
        let get_render_index_from_id = Rc::new(|id: Option<String>| {
            id.and_then(|s| s.strip_prefix("test-").and_then(|n| n.parse().ok()))
        });
        
        let get_id_from_render_index = Rc::new(|index: Option<usize>| {
            index.map(|i| format!("test-{}", i))
        });
        
        let get_local_index_from_id = get_render_index_from_id.clone();
        let get_id_from_local_index = get_id_from_render_index.clone();
        
        let get_scroll_offset_px = Rc::new(|index: usize| Some(index as f64 * 50.0));
        
        UseFocusFnParams {
            focus,
            select_by_id,
            get_render_index_from_id,
            get_id_from_render_index,
            get_local_index_from_id,
            get_id_from_local_index,
            get_scroll_offset_px,
            max_render_index: 10,
            max_local_index: 10,
            select_many: false,
        }
    }
    
    #[test]
    fn test_use_focus_fn_basic() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let params = create_test_params();
            let focus_fn = use_focus_fn(params);
            
            // Check initial state
            assert_eq!(focus_fn.id, Some("test-1".to_string()));
            
            rsx! { div { "Focus function test" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_get_molded_index() {
        assert_eq!(get_molded_index(0, 5, 10), 5);
        assert_eq!(get_molded_index(0, 15, 10), 9);
        assert_eq!(get_molded_index(2, 1, 10), 2);
        assert_eq!(get_molded_index(0, 5, 0), 0);
    }
    
    #[test]
    fn test_get_overflow_floored_index() {
        assert_eq!(get_overflow_floored_index(5, 10), 5);
        assert_eq!(get_overflow_floored_index(15, 10), 0);
        assert_eq!(get_overflow_floored_index(0, 10), 0);
    }
    
    #[test]
    fn test_focus_by_id_options_default() {
        let options = FocusByIdOptions::default();
        assert_eq!(options.without_scroll, Some(false));
        assert!(options.with_select.is_none());
    }
    
    #[test]
    fn test_focus_by_render_index_options_default() {
        let options = FocusByRenderIndexOptions::default();
        assert_eq!(options.fallback, Some(false));
        assert!(options.molded.is_none());
        assert!(options.base.is_none());
    }
    
    #[test]
    fn test_focus_by_local_index_options_default() {
        let options = FocusByLocalIndexOptions::default();
        assert_eq!(options.molded, Some(true));
        assert_eq!(options.floor_overflow, Some(false));
        assert!(options.base.is_none());
    }
}
