use std::collections::HashMap;

pub fn slices_to_map<T, U>(keys: &[T], values: &[U]) -> HashMap<&T, &U> {
    let mut map = HashMap::new();
    let size = keys.len().min(values.len());

    for i in 0..size {
        map.insert(&keys[i], &values[i]);
    }

    map
}
