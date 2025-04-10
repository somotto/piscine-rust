pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(arr.len() + 1);
    
    let mut current_sum: u64 = arr.iter().sum();
    result.push(current_sum);
    
    for &value in arr {
        current_sum -= value;
        result.push(current_sum);
    }
    
    result
}

// pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
//     let mut result = Vec::with_capacity(arr.len() + 1);
//     let mut sum = 0;

//     for &value in arr.iter().rev() {
//         result.push(sum);
//         sum += value;
//     }
//     result.push(sum); 

//     result.reverse();
//     result
// }

