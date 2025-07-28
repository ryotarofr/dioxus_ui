use std::collections::HashMap;

pub fn get_mapped_object<T, K, V>(
    obj: HashMap<K, T>,
    mapper: impl Fn((&K, &T), usize) -> V,
) -> HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
{
    obj.iter()
        .enumerate()
        .map(|(index, (key, value))| {
            (key.clone(), mapper((key, value), index))
        })
        .collect()
}
