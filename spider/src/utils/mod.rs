use crate::utils::errors::Error;
use async_trait::async_trait;

pub mod cve_details;
pub mod github;
pub mod quotes;

pub mod errors;

#[async_trait]
pub trait Spider: Send + Sync {
    type Item;

    fn name(&self) -> String;
    fn start_urls(&self) -> Vec<String>;
    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>),  Error>;
    async fn process(&self, item: Self::Item) -> Result<(),  Error>;
}