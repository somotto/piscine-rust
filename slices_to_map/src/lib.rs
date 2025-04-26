use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T: Eq + Hash, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut map = HashMap::new();
    let size = keys.len().min(values.len());

    for i in 0..size {
        map.insert(&keys[i], &values[i]);
    }

    map
}