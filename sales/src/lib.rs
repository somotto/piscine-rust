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

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        // Find the product in the store
        if let Some(product) = s.products.iter().find(|(name, _)| *name == ele) {
            // Add the product to the cart
            self.items.push((product.0.clone(), product.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // Extract only the prices from the items
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        
        // Sort prices in ascending order
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Calculate total original price
        let total_original: f32 = prices.iter().sum();
        
        // Calculate price after discount (every third item free)
        let mut total_after_discount = 0.0;
        for (i, &price) in prices.iter().enumerate() {
            if (i + 1) % 3 != 0 {
                total_after_discount += price;
            }
        }
        
        // Calculate discount ratio
        let discount_ratio = if total_original > 0.0 {
            total_after_discount / total_original
        } else {
            1.0
        };
        
        // Apply discount proportionally to each item
        let receipt: Vec<f32> = prices.iter()
            .map(|&price| (price * discount_ratio * 100.0).round() / 100.0)
            .collect();
        
        // Update the receipt field
        self.receipt = receipt.clone();
        
        receipt
    }
}