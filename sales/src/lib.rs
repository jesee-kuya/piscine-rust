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
        if let Some((_, price)) = s.products.iter().find(|(name, _)| name == &ele) {
            self.items.push((ele, *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let original_prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();

        let mut discount = 0.0;
        for chunk in original_prices.chunks(3) {
            if chunk.len() >= 3 {
                if let Some(min_price) = chunk.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
                    discount += min_price;
                }
            }
        }

        let original_sum: f32 = original_prices.iter().sum();
        let ratio = if original_sum == 0.0 {
            0.0
        } else {
            (original_sum - discount) / original_sum
        };

        let adjusted_prices: Vec<f32> = original_prices
            .iter()
            .map(|p| {
                let adjusted = p * ratio;
                (adjusted * 100.0).round() / 100.0
            })
            .collect();

        let mut receipt = adjusted_prices.clone();
        receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt = receipt.clone();
        receipt
    }
}
