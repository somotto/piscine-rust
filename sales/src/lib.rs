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
        
        // Calculate total cost before discount
        let original_total: f32 = prices.iter().sum();
        
        // Calculate total discount (every third item free)
        let free_items: Vec<f32> = prices.iter()
            .enumerate()
            .filter(|(i, _)| (i + 1) % 3 == 0)
            .map(|(_, &price)| price)
            .collect();
        
        let discount_total: f32 = free_items.iter().sum();
        
        // Calculate discount percentage
        let discount_percentage = if original_total > 0.0 {
            discount_total / original_total
        } else {
            0.0
        };
        
        // Apply discount to all items proportionally
        let receipt: Vec<f32> = prices.iter()
            .map(|&price| {
                let discounted_price = price * (1.0 - discount_percentage);
                // Round to 2 decimal places
                (discounted_price * 100.0).round() / 100.0
            })
            .collect();
        
        // Store the receipt
        self.receipt = receipt.clone();
        
        receipt
    }
}