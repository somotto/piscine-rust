pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() <= 1 {
        return Vec::new();
    }

    let total_product: usize = arr.iter().product();
    arr.iter().map(|&x| total_product / x).collect()
}
