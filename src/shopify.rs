use std::error::Error;

pub struct Shopify {
    api_key: String,
    pub data: ShopifyData
}

#[derive(Default)]
pub struct ShopifyData {
    pub today_sales: String,
    pub monthly_sales: String,
    pub today_visitors: String,
    pub monthly_visitors: String,
    pub today_orders: String,
    pub monthly_orders: String,
    pub current_active_sessions: String,
}

impl Shopify {
    pub fn new(api_key: String) -> Self {
        Self { api_key, data: ShopifyData::default() }
    }

    pub fn get_new_data(&mut self) -> Result<(), Box<dyn Error>> {
        self.data = ShopifyData::default();
        Ok(())
    }

}