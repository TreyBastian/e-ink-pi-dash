use std::time::Duration;
use crate::shopify::Shopify;

pub struct App {
    pub shopify_client: Shopify,
    pub update_interval: Duration,
}

impl App {
    pub fn new(shopify_client: Shopify, update_interval: Duration) -> Self {
        Self{shopify_client, update_interval}
    }
}