use std::error::Error;
use std::time::Duration;
use e_ink_pi_dash::app::App;
use e_ink_pi_dash::shopify::Shopify;


fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let update_interval = std::env::var("UPDATE_INTERVAL").expect("UPDATE_INTERVAL must be defined").parse::<u64>().expect("UPDATE_INTERVAL must be a number");
    let shopify_api_key = std::env::var("SHOPIFY_API_KEY").expect("Missing API Key");

    let mut app = App::new(Shopify::new(shopify_api_key), Duration::from_secs(update_interval));

    app.run()?;

    Ok(())
}


