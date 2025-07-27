use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum NestedValue {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<NestedValue>),
    Object(HashMap<String, NestedValue>),
    Null,
}

impl From<Value> for NestedValue {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => NestedValue::String(s),
            Value::Number(n) => NestedValue::Number(n.as_f64().unwrap_or(0.0)),
            Value::Bool(b) => NestedValue::Bool(b),
            Value::Array(arr) => {
                NestedValue::Array(arr.into_iter().map(NestedValue::from).collect())
            }
            Value::Object(obj) => {
                NestedValue::Object(
                    obj.into_iter()
                        .map(|(k, v)| (k, NestedValue::from(v)))
                        .collect(),
                )
            }
            Value::Null => NestedValue::Null,
        }
    }
}

impl From<String> for NestedValue {
    fn from(value: String) -> Self {
        NestedValue::String(value)
    }
}

impl From<&str> for NestedValue {
    fn from(value: &str) -> Self {
        NestedValue::String(value.to_string())
    }
}

impl From<f64> for NestedValue {
    fn from(value: f64) -> Self {
        NestedValue::Number(value)
    }
}

impl From<i32> for NestedValue {
    fn from(value: i32) -> Self {
        NestedValue::Number(value as f64)
    }
}

impl From<bool> for NestedValue {
    fn from(value: bool) -> Self {
        NestedValue::Bool(value)
    }
}

impl<T> From<Vec<T>> for NestedValue
where
    T: Into<NestedValue>,
{
    fn from(value: Vec<T>) -> Self {
        NestedValue::Array(value.into_iter().map(|v| v.into()).collect())
    }
}

impl<T> From<HashMap<String, T>> for NestedValue
where
    T: Into<NestedValue>,
{
    fn from(value: HashMap<String, T>) -> Self {
        NestedValue::Object(
            value.into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect()
        )
    }
}

/// Example
/// ```rust
/// let data = {"user": {"profile": {"name": "Alice"}}}
/// let name = data.get_nested_value(&["user", "profile", "name"]);
/// println!("{:?}", name); // NestedValue::String("Alice")
/// ```
pub trait NestedValueOf {
    fn get_nested_value(&self, keys: &[&str]) -> Option<NestedValue>;
}

/// Setting nested values. 
/// If the path does not exist, intermediate objects are created as necessary.
pub trait NestedValueSetter {
    fn set_nested_value(&mut self, keys: &[&str], value: NestedValue) -> bool;
}

impl NestedValueOf for NestedValue {
    fn get_nested_value(&self, keys: &[&str]) -> Option<NestedValue> {
        if keys.is_empty() {
            return Some(self.clone());
        }

        match self {
            NestedValue::Object(obj) => {
                let key = keys[0];
                if let Some(value) = obj.get(key) {
                    if keys.len() == 1 {
                        Some(value.clone())
                    } else {
                        value.get_nested_value(&keys[1..])
                    }
                } else {
                    None
                }
            }
            NestedValue::Array(arr) => {
                if let Ok(index) = keys[0].parse::<usize>() {
                    if let Some(value) = arr.get(index) {
                        if keys.len() == 1 {
                            Some(value.clone())
                        } else {
                            value.get_nested_value(&keys[1..])
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl<T> NestedValueOf for HashMap<String, T>
where
    T: Clone + Into<NestedValue>,
{
    fn get_nested_value(&self, keys: &[&str]) -> Option<NestedValue> {
        if keys.is_empty() {
            return None;
        }

        let key = keys[0];
        if let Some(value) = self.get(key) {
            let nested_value = value.clone().into();
            if keys.len() == 1 {
                Some(nested_value)
            } else {
                nested_value.get_nested_value(&keys[1..])
            }
        } else {
            None
        }
    }
}

impl<T> NestedValueOf for Vec<T>
where
    T: Clone + Into<NestedValue>,
{
    fn get_nested_value(&self, keys: &[&str]) -> Option<NestedValue> {
        if keys.is_empty() {
            return None;
        }

        if let Ok(index) = keys[0].parse::<usize>() {
            if let Some(value) = self.get(index) {
                let nested_value = value.clone().into();
                if keys.len() == 1 {
                    Some(nested_value)
                } else {
                    nested_value.get_nested_value(&keys[1..])
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl NestedValueSetter for NestedValue {
    fn set_nested_value(&mut self, keys: &[&str], value: NestedValue) -> bool {
        if keys.is_empty() {
            *self = value;
            return true;
        }

        match self {
            NestedValue::Object(ref mut obj) => {
                let key = keys[0];
                if keys.len() == 1 {
                    obj.insert(key.to_string(), value);
                    true
                } else if let Some(nested) = obj.get_mut(key) {
                    nested.set_nested_value(&keys[1..], value)
                } else {
                    let mut new_nested = NestedValue::Object(HashMap::new());
                    let success = new_nested.set_nested_value(&keys[1..], value);
                    if success {
                        obj.insert(key.to_string(), new_nested);
                    }
                    success
                }
            }
            NestedValue::Array(ref mut arr) => {
                if let Ok(index) = keys[0].parse::<usize>() {
                    if keys.len() == 1 {
                        if index < arr.len() {
                            arr[index] = value;
                            true
                        } else {
                            false
                        }
                    } else if index < arr.len() {
                        arr[index].set_nested_value(&keys[1..], value)
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_nested_value_from_serde_json() {
        let json_value = json!({
            "name": "test",
            "age": 25,
            "active": true,
            "scores": [100, 85, 92],
            "profile": {
                "email": "test@example.com",
                "settings": {
                    "theme": "dark"
                }
            },
            "nullable": null
        });

        let nested_value = NestedValue::from(json_value);

        if let NestedValue::Object(obj) = nested_value {
            assert!(matches!(obj.get("name"), Some(NestedValue::String(s)) if s == "test"));
            assert!(matches!(obj.get("age"), Some(NestedValue::Number(n)) if *n == 25.0));
            assert!(matches!(obj.get("active"), Some(NestedValue::Bool(true))));
            assert!(matches!(obj.get("nullable"), Some(NestedValue::Null)));
            
            if let Some(NestedValue::Array(arr)) = obj.get("scores") {
                assert_eq!(arr.len(), 3);
                assert!(matches!(arr[0], NestedValue::Number(n) if n == 100.0));
            }
        }
    }

    #[test]
    fn test_get_nested_value_object() {
        let mut data = HashMap::new();
        data.insert("user".to_string(), NestedValue::Object({
            let mut user = HashMap::new();
            user.insert("name".to_string(), NestedValue::String("John".to_string()));
            user.insert("profile".to_string(), NestedValue::Object({
                let mut profile = HashMap::new();
                profile.insert("email".to_string(), NestedValue::String("john@example.com".to_string()));
                profile
            }));
            user
        }));

        let nested_value = NestedValue::Object(data);

        // Test single level access
        let result = nested_value.get_nested_value(&["user"]);
        assert!(matches!(result, Some(NestedValue::Object(_))));

        // Test nested access
        let result = nested_value.get_nested_value(&["user", "name"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "John"));

        // Test deep nested access
        let result = nested_value.get_nested_value(&["user", "profile", "email"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "john@example.com"));

        // Test non-existent key
        let result = nested_value.get_nested_value(&["user", "nonexistent"]);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_nested_value_array() {
        let data = NestedValue::Array(vec![
            NestedValue::String("first".to_string()),
            NestedValue::Object({
                let mut obj = HashMap::new();
                obj.insert("nested".to_string(), NestedValue::Number(42.0));
                obj
            }),
            NestedValue::Array(vec![
                NestedValue::String("nested_array_item".to_string())
            ])
        ]);

        // Test array index access
        let result = data.get_nested_value(&["0"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "first"));

        // Test nested object in array
        let result = data.get_nested_value(&["1", "nested"]);
        assert!(matches!(result, Some(NestedValue::Number(n)) if n == 42.0));

        // Test nested array in array
        let result = data.get_nested_value(&["2", "0"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "nested_array_item"));

        // Test invalid index
        let result = data.get_nested_value(&["5"]);
        assert!(result.is_none());

        // Test non-numeric index
        let result = data.get_nested_value(&["invalid"]);
        assert!(result.is_none());
    }

    #[test]
    fn test_hashmap_nested_value_of() {
        let mut data = HashMap::new();
        data.insert("key1".to_string(), "value1".to_string());

        let result = data.get_nested_value(&["key1"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "value1"));

        // Test with nested HashMap
        let mut nested_data: HashMap<String, HashMap<String, String>> = HashMap::new();
        nested_data.insert("outer".to_string(), HashMap::from([
            ("inner".to_string(), "value".to_string())
        ]));

        let result = nested_data.get_nested_value(&["outer", "inner"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "value"));
    }

    #[test]
    fn test_vec_nested_value_of() {
        let data = vec!["first".to_string(), "second".to_string(), "third".to_string()];

        let result = data.get_nested_value(&["0"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "first"));

        let result = data.get_nested_value(&["2"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "third"));

        let result = data.get_nested_value(&["5"]);
        assert!(result.is_none());
    }

    #[test]
    fn test_set_nested_value_object() {
        let mut data = NestedValue::Object({
            let mut obj = HashMap::new();
            obj.insert("user".to_string(), NestedValue::Object({
                let mut user = HashMap::new();
                user.insert("name".to_string(), NestedValue::String("John".to_string()));
                user
            }));
            obj
        });

        // Test setting existing value
        let success = data.set_nested_value(&["user", "name"], NestedValue::String("Jane".to_string()));
        assert!(success);
        
        let result = data.get_nested_value(&["user", "name"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "Jane"));

        // Test setting new nested value
        let success = data.set_nested_value(&["user", "age"], NestedValue::Number(30.0));
        assert!(success);
        
        let result = data.get_nested_value(&["user", "age"]);
        assert!(matches!(result, Some(NestedValue::Number(n)) if n == 30.0));

        // Test creating new nested structure
        let success = data.set_nested_value(&["settings", "theme", "color"], NestedValue::String("dark".to_string()));
        assert!(success);
        
        let result = data.get_nested_value(&["settings", "theme", "color"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "dark"));
    }

    #[test]
    fn test_set_nested_value_array() {
        let mut data = NestedValue::Array(vec![
            NestedValue::String("first".to_string()),
            NestedValue::Object(HashMap::new()),
            NestedValue::String("third".to_string())
        ]);

        // Test setting existing array element
        let success = data.set_nested_value(&["0"], NestedValue::String("updated_first".to_string()));
        assert!(success);
        
        let result = data.get_nested_value(&["0"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "updated_first"));

        // Test setting nested value in object within array
        let success = data.set_nested_value(&["1", "key"], NestedValue::String("value".to_string()));
        assert!(success);
        
        let result = data.get_nested_value(&["1", "key"]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "value"));

        // Test setting out-of-bounds array element
        let success = data.set_nested_value(&["10"], NestedValue::String("out_of_bounds".to_string()));
        assert!(!success);
    }

    #[test]
    fn test_set_nested_value_replace_root() {
        let mut data = NestedValue::String("original".to_string());

        // Test replacing root value
        let success = data.set_nested_value(&[], NestedValue::Number(42.0));
        assert!(success);
        
        assert!(matches!(data, NestedValue::Number(n) if n == 42.0));
    }

    #[test]
    fn test_edge_cases() {
        let data = NestedValue::String("not_a_container".to_string());

        // Test accessing non-container type
        let result = data.get_nested_value(&["key"]);
        assert!(result.is_none());

        // Test setting on non-container type
        let mut data = NestedValue::String("not_a_container".to_string());
        let success = data.set_nested_value(&["key"], NestedValue::String("value".to_string()));
        assert!(!success);

        // Test empty keys
        let data = NestedValue::String("test".to_string());
        let result = data.get_nested_value(&[]);
        assert!(matches!(result, Some(NestedValue::String(s)) if s == "test"));
    }
}
