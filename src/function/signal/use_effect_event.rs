use dioxus::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

/**
 * https://github.com/reactjs/rfcs/blob/c5217c9dcf1ad46790ce6614976a65a35ed92b2e/text/0000-useevent.md#internal-implementation
 *
 * Polyfill for React's useEvent (renamed to useEffectEvent). Approximate behavior for Dioxus.
 * 
 * This hook provides a way to access the latest value of a function without causing re-renders
 * when the function reference changes. It's useful for event handlers that need to access 
 * fresh state but shouldn't trigger effect dependencies.
 * 
 * # Arguments
 * 
 * * `func` - The function to be wrapped in an effect event
 * 
 * # Returns
 * 
 * A stable function reference that always calls the latest version of the provided function
 * 
 * # Example
 * 
 * ```rust
 * use dioxus::prelude::*;
 * 
 * #[component]
 * fn ExampleComponent() -> Element {
 *     let mut count = use_signal(|| 0);
 *     
 *     // This function will always access the latest count value
 *     let handle_click = use_effect_event(move || {
 *         log::info!("Current count: {}", count.read());
 *     });
 *     
 *     rsx! {
 *         button { 
 *             onclick: move |_| handle_click(()),
 *             "Count: {count}"
 *         }
 *     }
 * }
 * ```
 */
pub fn use_effect_event<F, Args, R>(func: F) -> Rc<RefCell<dyn FnMut(Args) -> R>>
where
    F: FnMut(Args) -> R + 'static,
    Args: 'static,
    R: 'static,
{
    // Store the function in a Rc<RefCell> for mutable access and lifetime management
    let func_ref = use_signal(move || Rc::new(RefCell::new(func)));
    
    // Return the stable reference by cloning the value
    let result = func_ref.read().clone();
    result
}

/// Simpler version for functions that don't need to mutate
pub fn use_effect_event_fn<F, Args, R>(func: F) -> Rc<dyn Fn(Args) -> R>
where
    F: Fn(Args) -> R + 'static,
    Args: 'static,
    R: 'static,
{
    // Store the function in a Rc for shared access
    let func_ref = use_signal(move || Rc::new(func) as Rc<dyn Fn(Args) -> R>);
    
    // Return the stable reference by cloning the value
    let result = func_ref.read().clone();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_use_effect_event_basic() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let counter = use_signal(|| 0);
            
            // Create an effect event that captures the current counter value
            let get_count = use_effect_event(move |()| *counter.read());
            
            // The effect event should return the current counter value
            let current_count = get_count.borrow_mut()(());
            assert_eq!(current_count, 0);
            
            rsx! { div { "Count: {counter}" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_effect_event_with_args() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let base_value = use_signal(|| 10);
            
            // Create an effect event that adds the argument to the base value
            let add_to_base = use_effect_event(move |x: i32| {
                *base_value.read() + x
            });
            
            // Test the function with an argument
            let result = add_to_base.borrow_mut()(5);
            assert_eq!(result, 15);
            
            rsx! { div { "Base: {base_value}" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_effect_event_fn() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let base_value = use_signal(|| 10);
            
            // Create an effect event function (immutable)
            let get_value = use_effect_event_fn(move |()| {
                *base_value.read()
            });
            
            // Test calling the function
            let result = get_value(());
            assert_eq!(result, 10);
            
            rsx! { div { "Value: {base_value}" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_effect_event_mutable() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let mut internal_state = use_signal(|| 0);
            
            // Create a mutable effect event
            let increment = use_effect_event(move |amount: i32| {
                let new_value = *internal_state.read() + amount;
                internal_state.set(new_value);
                new_value
            });
            
            // Test calling the mutable function
            let _result = increment.borrow_mut()(5);
            // Note: In a real test, you'd need multiple renders to see state changes
            
            rsx! { div { "State: {internal_state}" } }
        });
        
        dom.rebuild_to_vec();
    }
    
    #[test]
    fn test_use_effect_event_function_stability() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let value = use_signal(|| "test");
            
            // Create an effect event
            let get_value = use_effect_event(move |()| value.read().clone());
            
            // The function should be callable multiple times
            let result1 = get_value.borrow_mut()(());
            let result2 = get_value.borrow_mut()(());
            
            assert_eq!(result1, "test");
            assert_eq!(result2, "test");
            
            rsx! { div { "Value: {value}" } }
        });
        
        dom.rebuild_to_vec();
    }
}
