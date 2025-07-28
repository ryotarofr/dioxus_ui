/// Range function utility for generating ranges of numbers
/// 
/// This function creates a range similar to JavaScript's Array.from({length: n}, (_, i) => i)
/// or Python's range() function.
/// 
/// # Arguments
/// 
/// * `length` - The length of the range
/// * `from` - Optional starting value (defaults to 0)
/// 
/// # Returns
/// 
/// Vec<usize> containing the range values
/// 
/// # Examples
/// 
/// ```rust
/// use dioxus_ui::function::range::range;
/// 
/// // Basic range from 0 to 4
/// assert_eq!(range(5), vec![0, 1, 2, 3, 4]);
/// 
/// // Range from 2 to 6
/// assert_eq!(range_from(5, 2), vec![2, 3, 4, 5, 6]);
/// ```
pub fn range(length: usize) -> Vec<usize> {
    (0..length).collect()
}

/// Range function with custom starting point
pub fn range_from(length: usize, from: usize) -> Vec<usize> {
    (from..from + length).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_range_basic() {
        assert_eq!(range(0), Vec::<usize>::new());
        assert_eq!(range(1), vec![0usize]);
        assert_eq!(range(5), vec![0usize, 1, 2, 3, 4]);
    }
    
    #[test]
    fn test_range_from() {
        assert_eq!(range_from(0, 5), Vec::<usize>::new());
        assert_eq!(range_from(1, 5), vec![5usize]);
        assert_eq!(range_from(5, 2), vec![2usize, 3, 4, 5, 6]);
        assert_eq!(range_from(3, 10), vec![10usize, 11, 12]);
    }
}