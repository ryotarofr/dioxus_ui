use std::collections::HashMap;
use std::hash::Hash;

pub struct Objects;

impl Objects {
    pub fn keys<K, V>(map: &HashMap<K, V>) -> Vec<&K>
    where
        K: Hash + Eq,
    {
        map.keys().collect()
    }

    pub fn values<K, V>(map: &HashMap<K, V>) -> Vec<&V>
    where
        K: Hash + Eq,
    {
        map.values().collect()
    }

    pub fn entries<K, V>(map: &HashMap<K, V>) -> Vec<(&K, &V)>
    where
        K: Hash + Eq,
    {
        map.iter().collect()
    }

    pub fn from_entries<K, V>(entries: Vec<(K, V)>) -> HashMap<K, V>
    where
        K: Hash + Eq,
    {
        entries.into_iter().collect()
    }

    pub fn map<K, V, R, F>(map: &HashMap<K, V>, mapper: F) -> HashMap<K, R>
    where
        K: Hash + Eq + Clone,
        F: Fn((&K, &V), usize) -> R,
    {
        map.iter()
            .enumerate()
            .map(|(index, (k, v))| (k.clone(), mapper((k, v), index)))
            .collect()
    }

    pub fn filter<K, V, F>(map: &HashMap<K, V>, filter: F) -> HashMap<K, V>
    where
        K: Hash + Eq + Clone,
        V: Clone,
        F: Fn((&K, &V), usize) -> bool,
    {
        map.iter()
            .enumerate()
            .filter(|(index, (k, v))| filter((k, v), *index))
            .map(|(_, (k, v))| (k.clone(), v.clone()))
            .collect()
    }

    pub fn find<K, V, F>(map: &HashMap<K, V>, finder: F) -> Option<(&K, &V)>
    where
        K: Hash + Eq,
        F: Fn((&K, &V), usize) -> bool,
    {
        map.iter()
            .enumerate()
            .find(|(index, (k, v))| finder((k, v), *index))
            .map(|(_, entry)| entry)
    }

    pub fn required<K, V>(map: &HashMap<K, Option<V>>) -> Option<HashMap<K, V>>
    where
        K: Hash + Eq + Clone,
        V: Clone,
    {
        if map.values().any(|v| v.is_none()) {
            return None;
        }
        
        Some(
            map.iter()
                .filter_map(|(k, v)| v.as_ref().map(|val| (k.clone(), val.clone())))
                .collect()
        )
    }
}
