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
        let mut discounted = Vec::new();
        let mut i = 0;
    
        while i + 2 < self.items.len() {
            // Take next 3 items
            let group = vec![
                self.items[i].1,
                self.items[i + 1].1,
                self.items[i + 2].1,
            ];
    
            let min_price = group.iter().cloned().reduce(f32::min).unwrap();
            let total: f32 = group.iter().sum();
            let new_total = total - min_price;
            let factor = new_total / total;
    
            for &price in &group {
                let adjusted = (price * factor * 100.0).round() / 100.0;
                discounted.push(adjusted);
            }
    
            i += 3;
        }
    
        // Remaining items
        for j in i..self.items.len() {
            let price = self.items[j].1;
            discounted.push((price * 100.0).round() / 100.0);
        }
    
        self.receipt = discounted.clone();
        discounted
    }
        
}
