use dioxus::prelude::*;
use crate::types::nested_value_of::{NestedValue, NestedValueOf, NestedValueSetter};

/// Function for generating setter functions for partialized `Signal`
/// 
/// Dioxus' Signal provides functionality equivalent to React's useState.
/// You can create setter functions that update only specific fields of a nested object.
///
/// # Arguments
///
/// * `signal` - The Signal to update
/// * `keys` - The path to the nested keys to access
/// 
/// # Returns
/// 
/// A closure that updates the value at the specified key path
/// 
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// 
/// #[component]
/// fn MyComponent() -> Element {
///     let mut user_signal = use_signal(|| User {
///         profile: Profile {
///             name: "John".to_string(),
///             email: "john@example.com".to_string(),
///         },
///         settings: Settings {
///             theme: "light".to_string(),
///         },
///     });
/// 
///     // Create a setter function that updates only the name.
///     let set_name = partialize_set_state_deep(user_signal, &["profile", "name"]);
///     
///     // usage
///     set_name("Jane".to_string());
/// 
///     rsx! {
///         div { "User: {user_signal.read().profile.name}" }
///     }
/// }
/// ```
pub fn partialize_set_state_deep<'a, T>(
    signal: Signal<T>,
    keys: &'a [&'a str],
) -> impl Fn(NestedValue) + 'a
where
    T: Clone + NestedValueOf + NestedValueSetter + 'static,
{
    move |new_value: NestedValue| {
        let mut signal = signal;
        signal.with_mut(|current| {
            current.set_nested_value(keys, new_value);
        });
    }
}

/// Single-level partialization setter function
///
/// Create a simple setter function for updating the direct fields of an object.
///
/// # Arguments
///
/// * `signal` - The Signal to update
/// * `key` - The key to access
/// 
/// # Returns
///
/// A closure that updates the value at the specified key
///
/// # Example
/// 
/// ```rust
/// use dioxus::prelude::*;
/// 
/// #[component]
/// fn MyComponent() -> Element {
///     let mut config = use_signal(|| Config {
///         theme: "light".to_string(),
///         language: "en".to_string(),
///     });
/// 
///     // Create a setter function that updates only the theme.
///     let set_theme = partialize_set_state(config, "theme");
///     
///     // usage
///     set_theme("dark".to_string());
/// 
///     rsx! {
///         div { "Theme: {config.read().theme}" }
///     }
/// }
/// ```
pub fn partialize_set_state<'a, T>(
    signal: Signal<T>,
    key: &'a str,
) -> impl Fn(NestedValue) + 'a
where
    T: Clone + NestedValueOf + NestedValueSetter + 'static,
{
    move |new_value: NestedValue| {
        let mut signal = signal;
        signal.with_mut(|current| {
            current.set_nested_value(&[key], new_value);
        });
    }
}

/// Abstracting type conversion to NestedValue
/// 
/// Example:
/// 
/// ```rust
/// let nested1 = create_nested_value("hello");
/// 
/// let nested2 = create_nested_value(42);
/// 
/// let nested3 = create_nested_value(my_custom_value);
/// ```
pub fn create_nested_value<T>(value: T) -> NestedValue
where
    T: Into<NestedValue>,
{
    value.into()
}

/// Convenience functions for string values
pub fn partialize_set_state_string<'a, T>(
    signal: Signal<T>,
    key: &'a str,
) -> impl Fn(String) + 'a
where
    T: Clone + NestedValueOf + NestedValueSetter + 'static,
{
    let setter = partialize_set_state(signal, key);
    move |value: String| {
        setter(NestedValue::String(value));
    }
}

/// Convenience functions for numerical values
pub fn partialize_set_state_number<'a, T>(
    signal: Signal<T>,
    key: &'a str,
) -> impl Fn(f64) + 'a
where
    T: Clone + NestedValueOf + NestedValueSetter + 'static,
{
    let setter = partialize_set_state(signal, key);
    move |value: f64| {
        setter(NestedValue::Number(value));
    }
}

/// Convenience functions for Boolean values
pub fn partialize_set_state_bool<'a, T>(
    signal: Signal<T>,
    key: &'a str,
) -> impl Fn(bool) + 'a
where
    T: Clone + NestedValueOf + NestedValueSetter + 'static,
{
    let setter = partialize_set_state(signal, key);
    move |value: bool| {
        setter(NestedValue::Bool(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::types::nested_value_of::NestedValue;

    #[derive(Clone, Debug, PartialEq)]
    struct TestUser {
        name: String,
        age: i32,
        profile: TestProfile,
    }

    #[derive(Clone, Debug, PartialEq)]
    struct TestProfile {
        email: String,
        bio: String,
    }

    impl NestedValueOf for TestUser {
        fn get_nested_value(&self, keys: &[&str]) -> Option<NestedValue> {
            if keys.is_empty() {
                return None;
            }

            match keys[0] {
                "name" => {
                    if keys.len() == 1 {
                        Some(NestedValue::String(self.name.clone()))
                    } else {
                        None
                    }
                }
                "age" => {
                    if keys.len() == 1 {
                        Some(NestedValue::Number(self.age as f64))
                    } else {
                        None
                    }
                }
                "profile" => {
                    if keys.len() == 1 {
                        Some(NestedValue::Object({
                            let mut map = HashMap::new();
                            map.insert("email".to_string(), NestedValue::String(self.profile.email.clone()));
                            map.insert("bio".to_string(), NestedValue::String(self.profile.bio.clone()));
                            map
                        }))
                    } else {
                        self.profile.get_nested_value(&keys[1..])
                    }
                }
                _ => None,
            }
        }
    }

    impl NestedValueOf for TestProfile {
        fn get_nested_value(&self, keys: &[&str]) -> Option<NestedValue> {
            if keys.is_empty() {
                return None;
            }

            match keys[0] {
                "email" => {
                    if keys.len() == 1 {
                        Some(NestedValue::String(self.email.clone()))
                    } else {
                        None
                    }
                }
                "bio" => {
                    if keys.len() == 1 {
                        Some(NestedValue::String(self.bio.clone()))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }

    impl NestedValueSetter for TestUser {
        fn set_nested_value(&mut self, keys: &[&str], value: NestedValue) -> bool {
            if keys.is_empty() {
                return false;
            }

            match keys[0] {
                "name" => {
                    if keys.len() == 1 {
                        if let NestedValue::String(s) = value {
                            self.name = s;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "age" => {
                    if keys.len() == 1 {
                        if let NestedValue::Number(n) = value {
                            self.age = n as i32;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "profile" => {
                    if keys.len() == 1 {
                        false // Cannot replace entire profile in this simple test
                    } else {
                        self.profile.set_nested_value(&keys[1..], value)
                    }
                }
                _ => false,
            }
        }
    }

    impl NestedValueSetter for TestProfile {
        fn set_nested_value(&mut self, keys: &[&str], value: NestedValue) -> bool {
            if keys.is_empty() {
                return false;
            }

            match keys[0] {
                "email" => {
                    if keys.len() == 1 {
                        if let NestedValue::String(s) = value {
                            self.email = s;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "bio" => {
                    if keys.len() == 1 {
                        if let NestedValue::String(s) = value {
                            self.bio = s;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
    }

    #[test]
    fn test_partialize_set_state_single_level() {
        use dioxus::prelude::*;
        
        // Create a virtual DOM for testing
        let mut dom = VirtualDom::new(|| {
            let user_signal = use_signal(|| TestUser {
                name: "John".to_string(),
                age: 30,
                profile: TestProfile {
                    email: "john@example.com".to_string(),
                    bio: "Software developer".to_string(),
                },
            });

            let set_name = partialize_set_state(user_signal, "name");
            set_name(NestedValue::String("Jane".to_string()));

            assert_eq!(user_signal.read().name, "Jane");
            assert_eq!(user_signal.read().age, 30); // Other fields unchanged

            rsx! { div {} }
        });
        
        dom.rebuild_to_vec();
    }

    #[test]
    fn test_partialize_set_state_deep() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let user_signal = use_signal(|| TestUser {
                name: "John".to_string(),
                age: 30,
                profile: TestProfile {
                    email: "john@example.com".to_string(),
                    bio: "Software developer".to_string(),
                },
            });

            let set_email = partialize_set_state_deep(user_signal, &["profile", "email"]);
            set_email(NestedValue::String("jane@example.com".to_string()));

            assert_eq!(user_signal.read().profile.email, "jane@example.com");
            assert_eq!(user_signal.read().name, "John"); // Other fields unchanged
            assert_eq!(user_signal.read().profile.bio, "Software developer"); // Other nested fields unchanged

            rsx! { div {} }
        });
        
        dom.rebuild_to_vec();
    }

    #[test]
    fn test_convenience_functions() {
        use dioxus::prelude::*;
        
        let mut dom = VirtualDom::new(|| {
            let user_signal = use_signal(|| TestUser {
                name: "John".to_string(),
                age: 30,
                profile: TestProfile {
                    email: "john@example.com".to_string(),
                    bio: "Software developer".to_string(),
                },
            });

            let set_name = partialize_set_state_string(user_signal, "name");
            let set_age = partialize_set_state_number(user_signal, "age");

            set_name("Alice".to_string());
            set_age(25.0);

            assert_eq!(user_signal.read().name, "Alice");
            assert_eq!(user_signal.read().age, 25);

            rsx! { div {} }
        });
        
        dom.rebuild_to_vec();
    }
}