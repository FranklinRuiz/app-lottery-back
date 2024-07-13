use std::collections::HashSet;
use std::sync::Arc;
use chrono::{DateTime};
use crate::infrastructure::tiktok_api::TikTokApi;
use crate::domain::entities::comment::{ApiResponse, TransformedApiResponse, TransformedComment, TransformedUser, TransformedThumb};
use reqwest::Error;

pub struct CommentService {
    api: Arc<TikTokApi>,
}

impl CommentService {
    pub fn new(api: Arc<TikTokApi>) -> Self {
        CommentService { api }
    }

    pub async fn get_comments(&self, unique_id: &str) -> Result<TransformedApiResponse, Error> {

        let mut cursor = 0;
        let mut all_comments = Vec::new();
        let mut id = HashSet::new();

        loop {
            let api_url = self.api.get_comments_url(unique_id, cursor);
            let api_response: ApiResponse = self.api.fetch_comments(&api_url).await?;

            if let Some(comments) = api_response.comments {
                for comment in comments {
                    if id.insert(comment.cid.clone()) {
                        all_comments.push(comment);
                    }
                }
                cursor = api_response.cursor;
            } else {
                break;
            }
        }

        all_comments.sort_by_key(|comment| -comment.create_time);

        let transformed_comments: Vec<TransformedComment> = all_comments.into_iter().map(|comment| {

            let datetime = DateTime::from_timestamp(comment.create_time, 0).unwrap().format("%d/%m/%Y");
            let formatted_date = datetime.to_string();

            TransformedComment {
                comment: comment.text,
                user: TransformedUser {
                    thumb: TransformedThumb {
                        url_list: comment.user.avatar_thumb.url_list,
                    },
                    nickname: comment.user.nickname,
                    unique_id: comment.user.unique_id,
                },
                create_time: formatted_date,
                id: comment.cid,
                like: comment.digg_count,
            }
        }).collect();

        Ok(TransformedApiResponse {
            comments: transformed_comments,
        })
    }
}
