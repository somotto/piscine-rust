pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(arr.len() + 1);

    for i in 0..=arr.len() {
        let partition_sum = arr.get(0..arr.len().saturating_sub(i))
                               .unwrap_or(&[])
                               .iter()
                               .sum();
        result.push(partition_sum);
    }

    result
}
