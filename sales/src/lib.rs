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
        let mut discounted_items: Vec<(String, f32)> = Vec::new();
        let mut i = 0;

        while i + 2 < self.items.len() {
            let group = &self.items[i..i + 3];
            let prices: Vec<f32> = group.iter().map(|(_, p)| *p).collect();

            let min_price = prices.iter().cloned().reduce(f32::min).unwrap();
            let total: f32 = prices.iter().sum();
            let new_total = total - min_price;
            let factor = new_total / total;

            for (name, price) in group {
                let adjusted = (price * factor * 100.0).round() / 100.0;
                discounted_items.push((name.clone(), adjusted));
            }

            i += 3;
        }

        for j in i..self.items.len() {
            let (name, price) = &self.items[j];
            discounted_items.push((name.clone(), (*price * 100.0).round() / 100.0));
        }

        // Sort by product name (like the test expects)
        discounted_items.sort_by(|a, b| a.0.cmp(&b.0));

        let receipt: Vec<f32> = discounted_items.iter().map(|(_, p)| *p).collect();
        self.receipt = receipt.clone();
        receipt
    }
}