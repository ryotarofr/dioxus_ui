pub trait PartializedTuple<T> {
    fn get_partialized_tuples(&self) -> Vec<Vec<T>>;
}

impl<T: Clone> PartializedTuple<T> for Vec<T> {
    fn get_partialized_tuples(&self) -> Vec<Vec<T>> {
        if self.is_empty() {
            return vec![];
        }

        let mut result = Vec::new();
        
        for i in 1..=self.len() {
            result.push(self[0..i].to_vec());
        }
        
        result
    }
}

impl<T: Clone> PartializedTuple<T> for [T] {
    fn get_partialized_tuples(&self) -> Vec<Vec<T>> {
        if self.is_empty() {
            return vec![];
        }

        let mut result = Vec::new();
        
        for i in 1..=self.len() {
            result.push(self[0..i].to_vec());
        }
        
        result
    }
}
