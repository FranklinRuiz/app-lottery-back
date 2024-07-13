use std::sync::Arc;
use crate::application::services::comment_service::CommentService;
use crate::infrastructure::tiktok_api::TikTokApi;

pub async fn configure_services() -> Arc<CommentService> {
    let comment_api_url = "https://www.tiktok.com/api/comment/list/?aid=1988&app_language=ja-JP&app_name=tiktok_web&browser_language=es&browser_name=Mozilla&browser_online=true&browser_platform=Win32&browser_version=5.0%20(Windows%20NT%2010.0;%20Win64;%20x64)%20AppleWebKit/537.36%20(KHTML,%20like%20Gecko)%20Chrome/126.0.0.0%20Safari/537.36&channel=tiktok_web&cookie_enabled=true&count=20".to_string();
    let comment_api = Arc::new(TikTokApi::new(comment_api_url));
    let comment_service = Arc::new(CommentService::new(comment_api));

    comment_service
}