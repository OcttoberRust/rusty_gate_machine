use std::time::Duration;
use tokio::time::sleep;

mod scraper;
mod monitor;
mod notifications;
mod purchase;
mod config;

#[tokio::main]
async fn main() {
    // Load configuration (e.g., event IDs, price thresholds, API keys)


    //loop
        // Fetch ticket listings from different platforms
        // Combine results
        // Monitor prices and find deals
        // Send notification

            // Auto-purchase if enabled
            //Exit loop after success


    // Sleep before checking again

}
