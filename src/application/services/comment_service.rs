use std::collections::HashSet;
use std::sync::Arc;
use chrono::{DateTime};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use crate::infrastructure::tiktok_api::TikTokApi;
use crate::domain::entities::comment::{ApiResponse, TransformedApiResponse, TransformedComment, TransformedUser, TransformedThumb, LotteryResponse};
use reqwest::Error;

pub struct CommentService {
    api: Arc<TikTokApi>,
}

impl CommentService {
    pub fn new(api: Arc<TikTokApi>) -> Self {
        CommentService { api }
    }

    fn extract_video_id(url: &str) -> String {
        let pattern = r"video/([^?]+)";
        let re = Regex::new(pattern).unwrap();

        if let Some(caps) = re.captures(url) {
            return caps.get(1).map_or("video".to_string(), |m| m.as_str().to_string());
        } else {
            return "video".to_string();
        }
    }

    pub async fn get_comments(&self, url: &str) -> Result<TransformedApiResponse, Error> {
        let mut cursor = 0;
        let mut all_comments = Vec::new();
        let mut id = HashSet::new();
        let unique_id = Self::extract_video_id(url);

        loop {
            let api_url = self.api.get_comments_url(&*unique_id, cursor);
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

        let transformed_comments: Vec<TransformedComment> = all_comments
            .into_iter()
            .map(|comment| {

            let datetime = DateTime::from_timestamp(comment.create_time, 0)
                .unwrap()
                .format("%d/%m/%Y");

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

    pub async fn get_lottery(&self, mut participants: Vec<String>, num_winners: usize) -> Result<LotteryResponse, Error> {
        let mut rng = thread_rng();

        participants.shuffle(&mut rng);

        let winners: Vec<String> = participants
            .into_iter()
            .take(num_winners)
            .collect();

        Ok(LotteryResponse {
            winners,
        })
    }
}
