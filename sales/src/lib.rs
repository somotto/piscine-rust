#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, ele: String) {
        if let Some((_, price)) = store.products.iter().find(|(name, _)| *name == ele) {
            self.items.push((ele, *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        let mut discounted = Vec::new();
    
        // Sort to ensure consistent grouping of triplets
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
        let mut i = 0;
        while i + 2 < prices.len() {
            let group = vec![prices[i], prices[i + 1], prices[i + 2]];
            let min_price = group.iter().cloned().reduce(f32::min).unwrap();
    
            let total: f32 = group.iter().sum();
            let new_total = total - min_price;
    
            let factor = new_total / total;
    
            for &p in &group {
                let adjusted = (p * factor * 100.0).round() / 100.0;
                discounted.push(adjusted);
            }
    
            i += 3;
        }
    
        // Handle remaining items
        for j in i..prices.len() {
            discounted.push((prices[j] * 100.0).round() / 100.0);
        }
    
        discounted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = discounted.clone();
        discounted
    }    
}
