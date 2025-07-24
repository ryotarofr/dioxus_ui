use std::collections::HashMap;
use std::hash::Hash;
use serde_json::{Value, Map};

/// Merges multiple JSON values together recursively.
/// Objects are merged deeply, while arrays and primitives are replaced.
pub fn merge(objs: Vec<Value>) -> Value {
    if objs.is_empty() {
        return Value::Null;
    }
    
    if objs.len() == 1 {
        return objs[0].clone();
    }
    
    // Collect all keys from all objects
    let mut all_keys = std::collections::HashSet::new();
    for obj in &objs {
        if let Value::Object(map) = obj {
            for key in map.keys() {
                all_keys.insert(key.clone());
            }
        }
    }
    
    let mut result = Map::new();
    
    for key in all_keys {
        let merged_value = objs.iter().fold(None, |proc: Option<Value>, obj| {
            let val = match obj {
                Value::Object(map) => map.get(&key).cloned(),
                _ => None,
            };
            
            match val {
                None => proc,
                Some(val) => {
                    let mergeable = is_mergeable(&val) && 
                        proc.as_ref().is_none_or(is_mergeable);
                    
                    if !mergeable {
                        Some(val)
                    } else {
                        match proc {
                            None => Some(val),
                            Some(proc_val) => {
                                if let (Value::Object(_), Value::Object(_)) = 
                                    (&proc_val, &val) {
                                    Some(merge(vec![proc_val, val]))
                                } else {
                                    Some(val)
                                }
                            }
                        }
                    }
                }
            }
        });
        
        if let Some(value) = merged_value {
            result.insert(key, value);
        }
    }
    
    Value::Object(result)
}

/// Merges two JSON values together recursively.
pub fn merge_two(a: Value, b: Value) -> Value {
    merge(vec![a, b])
}

/// Merges multiple HashMaps together.
/// For non-object types, later values override earlier ones.
pub fn merge_hashmaps<K, V>(maps: Vec<HashMap<K, V>>) -> HashMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    let mut result = HashMap::new();
    
    for map in maps {
        for (key, value) in map {
            result.insert(key, value);
        }
    }
    
    result
}

/// Deeply merges two HashMaps containing Values.
pub fn deep_merge_hashmaps<K>(
    a: HashMap<K, Value>, 
    b: HashMap<K, Value>
) -> HashMap<K, Value>
where
    K: Clone + Hash + Eq,
{
    let mut result = a.clone();
    
    for (key, b_value) in b {
        match result.get(&key) {
            Some(a_value) => {
                if is_mergeable(a_value) && is_mergeable(&b_value) {
                    result.insert(key, merge_two(a_value.clone(), b_value));
                } else {
                    result.insert(key, b_value);
                }
            }
            None => {
                result.insert(key, b_value);
            }
        }
    }
    
    result
}

/// Checks if a JSON value can be merged (is an object).
fn is_mergeable(value: &Value) -> bool {
    matches!(value, Value::Object(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_merge_simple_objects() {
        let a = json!({"a": 1, "b": 2});
        let b = json!({"b": 3, "c": 4});
        let result = merge_two(a, b);
        let expected = json!({"a": 1, "b": 3, "c": 4});
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_merge_nested_objects() {
        let a = json!({"nested": {"a": 1, "b": 2}, "other": "value"});
        let b = json!({"nested": {"b": 3, "c": 4}});
        let result = merge_two(a, b);
        let expected = json!({"nested": {"a": 1, "b": 3, "c": 4}, "other": "value"});
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_merge_arrays_replace() {
        let a = json!({"arr": [1, 2, 3]});
        let b = json!({"arr": [4, 5]});
        let result = merge_two(a, b);
        let expected = json!({"arr": [4, 5]});
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_merge_multiple_objects() {
        let objs = vec![
            json!({"a": 1}),
            json!({"b": 2}),
            json!({"c": 3}),
        ];
        let result = merge(objs);
        let expected = json!({"a": 1, "b": 2, "c": 3});
        assert_eq!(result, expected);
    }
}
