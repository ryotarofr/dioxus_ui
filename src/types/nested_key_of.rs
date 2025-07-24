use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum NestedKey {
    /// HashMap keys
    String(String),
    /// Vec index
    Number(usize),
}

/// Full path to value
pub type NestedKeyPath = Vec<NestedKey>;

/// Get paths to all values from any data structure.
/// This trait is mainly used in the table_view component and serves the following purposes.
///   - Automatically flattens objects for table display.
///   - Sorting function for nested values.
/// 
/// Example of operation: Nested object data
/// {
///   "user": {
///     "name": "Alice",
///     "hobbies": ["reading", "coding"]
///   }
/// }
/// 
/// Calling get_nested_keys() on this object data:
/// [
///    ["user", "name"],           // Route to “Alice”
///    ["user", "hobbies", 0],     // Route to “reading”
///    ["user", "hobbies", 1]      // Route to “coding”
///  ]
pub trait NestedKeyOf {
    fn get_nested_keys(&self) -> Vec<NestedKeyPath>;
}

impl<T> NestedKeyOf for HashMap<String, T> 
where
    T: NestedKeyOf + Clone,
{
    fn get_nested_keys(&self) -> Vec<NestedKeyPath> {
        self.iter()
            .flat_map(|(key, value)| {
                let nested_keys = value.get_nested_keys();
                if nested_keys.is_empty() {
                    vec![vec![NestedKey::String(key.clone())]]
                } else {
                    nested_keys
                        .into_iter()
                        .map(|mut path| {
                            path.insert(0, NestedKey::String(key.clone()));
                            path
                        })
                        .collect()
                }
            })
            .collect()
    }
}

impl<T> NestedKeyOf for Vec<T>
where
    T: NestedKeyOf + Clone,
{
    fn get_nested_keys(&self) -> Vec<NestedKeyPath> {
        self.iter()
            .enumerate()
            .flat_map(|(index, value)| {
                let nested_keys = value.get_nested_keys();
                if nested_keys.is_empty() {
                    vec![vec![NestedKey::Number(index)]]
                } else {
                    nested_keys
                        .into_iter()
                        .map(|mut path| {
                            path.insert(0, NestedKey::Number(index));
                            path
                        })
                        .collect()
                }
            })
            .collect()
    }
}

impl NestedKeyOf for String {
    fn get_nested_keys(&self) -> Vec<NestedKeyPath> {
        vec![]
    }
}

impl NestedKeyOf for i32 {
    fn get_nested_keys(&self) -> Vec<NestedKeyPath> {
        vec![]
    }
}

impl NestedKeyOf for f64 {
    fn get_nested_keys(&self) -> Vec<NestedKeyPath> {
        vec![]
    }
}

impl NestedKeyOf for bool {
    fn get_nested_keys(&self) -> Vec<NestedKeyPath> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_primitive_types_return_empty_keys() {
        let string_val = "hello".to_string();
        let int_val = 42i32;
        let float_val = std::f64::consts::PI;
        let bool_val = true;

        assert_eq!(string_val.get_nested_keys(), Vec::<NestedKeyPath>::new());
        assert_eq!(int_val.get_nested_keys(), Vec::<NestedKeyPath>::new());
        assert_eq!(float_val.get_nested_keys(), Vec::<NestedKeyPath>::new());
        assert_eq!(bool_val.get_nested_keys(), Vec::<NestedKeyPath>::new());
    }

    #[test]
    fn test_hashmap_with_primitive_values() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), "Alice".to_string());
        map.insert("age".to_string(), "30".to_string());

        let keys = map.get_nested_keys();
        
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&vec![NestedKey::String("name".to_string())]));
        assert!(keys.contains(&vec![NestedKey::String("age".to_string())]));
    }

    #[test]
    fn test_vec_with_primitive_values() {
        let vec = vec!["first".to_string(), "second".to_string(), "third".to_string()];
        
        let keys = vec.get_nested_keys();
        
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&vec![NestedKey::Number(0)]));
        assert!(keys.contains(&vec![NestedKey::Number(1)]));
        assert!(keys.contains(&vec![NestedKey::Number(2)]));
    }

    #[test]
    fn test_nested_hashmap() {
        let mut inner_map = HashMap::new();
        inner_map.insert("city".to_string(), "Tokyo".to_string());
        inner_map.insert("country".to_string(), "Japan".to_string());

        let mut outer_map = HashMap::new();
        outer_map.insert("location".to_string(), inner_map);

        let keys = outer_map.get_nested_keys();
        
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&vec![
            NestedKey::String("location".to_string()),
            NestedKey::String("city".to_string())
        ]));
        assert!(keys.contains(&vec![
            NestedKey::String("location".to_string()),
            NestedKey::String("country".to_string())
        ]));
    }

    #[test]
    fn test_nested_vec() {
        let inner_vec1 = vec!["a".to_string(), "b".to_string()];
        let inner_vec2 = vec!["c".to_string()];
        let outer_vec = vec![inner_vec1, inner_vec2];

        let keys = outer_vec.get_nested_keys();
        
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&vec![NestedKey::Number(0), NestedKey::Number(0)]));
        assert!(keys.contains(&vec![NestedKey::Number(0), NestedKey::Number(1)]));
        assert!(keys.contains(&vec![NestedKey::Number(1), NestedKey::Number(0)]));
    }

    #[test]
    fn test_mixed_nested_structures() {
        let mut inner_map = HashMap::new();
        inner_map.insert("values".to_string(), vec!["x".to_string(), "y".to_string()]);

        let mut outer_map = HashMap::new();
        outer_map.insert("data".to_string(), inner_map);

        let keys = outer_map.get_nested_keys();
        
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&vec![
            NestedKey::String("data".to_string()),
            NestedKey::String("values".to_string()),
            NestedKey::Number(0)
        ]));
        assert!(keys.contains(&vec![
            NestedKey::String("data".to_string()),
            NestedKey::String("values".to_string()),
            NestedKey::Number(1)
        ]));
    }

    #[test]
    fn test_empty_containers() {
        let empty_map: HashMap<String, String> = HashMap::new();
        let empty_vec: Vec<String> = Vec::new();

        assert_eq!(empty_map.get_nested_keys(), Vec::<NestedKeyPath>::new());
        assert_eq!(empty_vec.get_nested_keys(), Vec::<NestedKeyPath>::new());
    }

    #[test]
    fn test_complex_nested_structure() {
        let mut level2_map = HashMap::new();
        level2_map.insert("item".to_string(), vec!["value1".to_string(), "value2".to_string()]);

        let mut level1_map = HashMap::new();
        level1_map.insert("nested".to_string(), level2_map);

        let vec_of_maps = vec![level1_map];

        let keys = vec_of_maps.get_nested_keys();
        
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&vec![
            NestedKey::Number(0),
            NestedKey::String("nested".to_string()),
            NestedKey::String("item".to_string()),
            NestedKey::Number(0)
        ]));
        assert!(keys.contains(&vec![
            NestedKey::Number(0),
            NestedKey::String("nested".to_string()),
            NestedKey::String("item".to_string()),
            NestedKey::Number(1)
        ]));
    }
}