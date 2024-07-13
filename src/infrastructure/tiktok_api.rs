use reqwest::Error;
use crate::domain::entities::comment::ApiResponse;

pub struct TikTokApi {
    base_url: String,
}

impl TikTokApi {
    pub fn new(base_url: String) -> Self {
        TikTokApi { base_url }
    }

    pub fn get_comments_url(&self, unique_id: &str, cursor: u32) -> String {
        format!("{}&aweme_id={}&cursor={}", self.base_url, unique_id, cursor)
    }

    pub async fn fetch_comments(&self, url: &str) -> Result<ApiResponse, Error> {
        let response = reqwest::get(url).await?;
        let api_response: ApiResponse = response.json().await?;
        Ok(api_response)
    }
}
