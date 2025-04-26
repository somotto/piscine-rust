pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let total_product: usize = arr.iter().product();
    arr.iter().map(|&x| total_product / x).collect()
}
