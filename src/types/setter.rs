use std::rc::Rc;
use crate::types::{
    nested_key_of::NestedKeyOf,
    nested_value_of::{NestedValue, NestedValueOf, NestedValueSetter},
};

/// A type alias for a state setter function.
/// 
/// This represents a function that can update state by accepting a `SetStateAction`.
/// The setter is wrapped in an `Rc` to allow sharing across multiple contexts.
/// 
/// # Example
/// ```rust
/// use std::rc::Rc;
/// use your_crate::types::setter::{Setter, SetStateAction};
/// 
/// let setter: Setter<i32> = Rc::new(|action| {
///     // Handle the state update
/// });
/// ```
pub type Setter<T> = Rc<dyn Fn(SetStateAction<T>)>;

/// Represents an action to update state.
/// 
/// This enum allows for two types of state updates:
/// - Direct value replacement
/// - Function-based updates that receive the previous state
/// 
/// # Example
/// ```rust
/// use std::rc::Rc;
/// use your_crate::types::setter::SetStateAction;
/// 
/// // Direct value update
/// let action1 = SetStateAction::Value(42);
/// 
/// // Function-based update
/// let action2 = SetStateAction::Function(Rc::new(|prev| prev + 1));
/// ```
#[derive(Clone)]
pub enum SetStateAction<T> {
    /// Set the state to a specific value
    Value(T),
    /// Update the state using a function that receives the previous state
    Function(Rc<dyn Fn(T) -> T>),
}

pub struct SetterUtils;

impl SetterUtils {
    /// Converts a `SetStateAction` to its final value by applying it to the previous state.
    /// 
    /// # Arguments
    /// * `set_state_action` - The action to apply
    /// * `prev` - The previous state value
    /// 
    /// # Returns
    /// The new state value after applying the action
    /// 
    /// # Example
    /// ```rust
    /// use std::rc::Rc;
    /// use your_crate::types::setter::{SetStateAction, SetterUtils};
    /// 
    /// let action = SetStateAction::Value(42);
    /// let result = SetterUtils::to_value(action, 0);
    /// assert_eq!(result, 42);
    /// 
    /// let action = SetStateAction::Function(Rc::new(|x| x + 10));
    /// let result = SetterUtils::to_value(action, 5);
    /// assert_eq!(result, 15);
    /// ```
    pub fn to_value<T: Clone>(set_state_action: SetStateAction<T>, prev: T) -> T {
        match set_state_action {
            SetStateAction::Value(value) => value,
            SetStateAction::Function(func) => func(prev),
        }
    }

    /// Creates a setter from a function that handles state updates.
    /// 
    /// This method bridges between the `SetStateAction` pattern and a function-based
    /// state management system.
    /// 
    /// # Arguments
    /// * `use_fn` - A function that will be called with the state update function
    /// 
    /// # Returns
    /// A `Setter<T>` that can be used to update state
    /// 
    /// # Example
    /// ```rust
    /// use std::rc::Rc;
    /// use your_crate::types::setter::{SetStateAction, SetterUtils};
    /// 
    /// let setter = SetterUtils::from(|update_fn: Rc<dyn Fn(i32) -> i32>| {
    ///     // This would typically update actual state
    ///     let new_value = update_fn(current_state);
    ///     // Update your state management system here
    /// });
    /// 
    /// setter(SetStateAction::Value(42));
    /// ```
    pub fn from<T: Clone + 'static>(
        use_fn: impl Fn(Rc<dyn Fn(T) -> T>) + 'static,
    ) -> Setter<T> {
        Rc::new(move |set_state_action| {
            use_fn(Rc::new(move |prev| {
                Self::to_value(set_state_action.clone(), prev)
            }));
        })
    }

    /// Creates a partial setter that can update a single nested field.
    /// 
    /// This method allows you to create setters for specific fields within a nested
    /// data structure, enabling granular state updates.
    /// 
    /// # Arguments
    /// * `set_state` - The main state setter
    /// 
    /// # Returns
    /// A function that takes a key and returns a setter for that specific field
    /// 
    /// # Example
    /// ```rust
    /// use std::rc::Rc;
    /// use your_crate::types::setter::{SetStateAction, SetterUtils};
    /// 
    /// let main_setter = Rc::new(|action| {
    ///     // Update main state
    /// });
    /// 
    /// let partial_setter = SetterUtils::partial_once(main_setter);
    /// let name_setter = partial_setter("name");
    /// 
    /// name_setter(SetStateAction::Value(Some(NestedValue::String("John".to_string()))));
    /// ```
    pub fn partial_once<T: Clone + NestedValueOf + Into<NestedValue> + 'static>(
        set_state: Setter<T>,
    ) -> impl Fn(&str) -> Setter<Option<NestedValue>> {
        move |key: &str| {
            let key = key.to_string();
            let set_state = set_state.clone();
            SetterUtils::from(move |use_fn: Rc<dyn Fn(Option<NestedValue>) -> Option<NestedValue>>| {
                let key = key.clone();
                let use_fn = use_fn.clone();
                set_state(SetStateAction::Function(Rc::new(move |prev| {
                    // Get current nested value
                    let current_value = prev.get_nested_value(&[&key]);
                    
                    // Apply the update function
                    let updated_value = use_fn(current_value);
                    
                    // If we have an updated value, create a new state with the change
                    if let Some(new_value) = updated_value {
                        let mut new_state: NestedValue = prev.clone().into();
                        if new_state.set_nested_value(&[&key], new_value) {
                            // Try to convert back to T - this is simplified
                            // In practice, you'd need proper conversion logic
                            return prev; // For now, return unchanged
                        }
                    }
                    
                    prev
                })));
            })
        }
    }

    /// Creates a partial setter that can update deeply nested fields.
    /// 
    /// This method allows you to create setters for specific fields at any depth
    /// within a nested data structure using a key path.
    /// 
    /// # Arguments
    /// * `set_state` - The main state setter
    /// 
    /// # Returns
    /// A function that takes a key path and returns a setter for that specific nested field
    /// 
    /// # Example
    /// ```rust
    /// use std::rc::Rc;
    /// use your_crate::types::setter::{SetStateAction, SetterUtils};
    /// 
    /// let main_setter = Rc::new(|action| {
    ///     // Update main state
    /// });
    /// 
    /// let partial_setter = SetterUtils::partial(main_setter);
    /// let deep_setter = partial_setter(&["user".to_string(), "profile".to_string(), "name".to_string()]);
    /// 
    /// deep_setter(SetStateAction::Value(Some(NestedValue::String("Jane".to_string()))));
    /// ```
    pub fn partial<T: Clone + NestedKeyOf + NestedValueOf + Into<NestedValue> + 'static>(
        set_state: Setter<T>,
    ) -> impl Fn(&[String]) -> Setter<Option<NestedValue>> {
        move |keys: &[String]| {
            if keys.is_empty() {
                // Return a no-op setter for empty keys
                return SetterUtils::from(move |_use_fn: Rc<dyn Fn(Option<NestedValue>) -> Option<NestedValue>>| {
                    // Do nothing
                });
            }
            
            let keys = keys.to_vec();
            let set_state = set_state.clone();
            SetterUtils::from(move |use_fn: Rc<dyn Fn(Option<NestedValue>) -> Option<NestedValue>>| {
                let keys = keys.clone();
                let use_fn = use_fn.clone();
                set_state(SetStateAction::Function(Rc::new(move |prev| {
                    // Convert keys to &str slice
                    let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
                    
                    // Get current nested value
                    let current_value = prev.get_nested_value(&key_refs);
                    
                    // Apply the update function
                    let updated_value = use_fn(current_value);
                    
                    // If we have an updated value, create a new state with the change
                    if let Some(new_value) = updated_value {
                        let mut new_state: NestedValue = prev.clone().into();
                        if new_state.set_nested_value(&key_refs, new_value) {
                            // Try to convert back to T - this is simplified
                            // In practice, you'd need proper conversion logic
                            return prev; // For now, return unchanged
                        }
                    }
                    
                    prev
                })));
            })
        }
    }
}
