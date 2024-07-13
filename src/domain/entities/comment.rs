use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub text: String,
    pub user: User,
    pub create_time: i64,
    pub cid: String,
    pub digg_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub avatar_thumb: AvatarThumb,
    pub nickname: String,
    pub unique_id: String,
}

#[derive(Deserialize, Debug)]
pub struct AvatarThumb {
    pub uri: String,
    pub url_list: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub comments: Option<Vec<Comment>>,
    pub cursor: u32,
    pub total: u32,
}

// Estructura para la respuesta transformada
#[derive(Serialize, Debug)]
pub struct TransformedComment {
    pub comment: String,
    pub user: TransformedUser,
    pub create_time: String,
    pub id: String,
    pub like: i32,
}

#[derive(Serialize, Debug)]
pub struct TransformedUser {
    pub thumb: TransformedThumb,
    pub nickname: String,
    pub unique_id: String,
}

#[derive(Serialize, Debug)]
pub struct TransformedThumb {
    pub url_list: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct TransformedApiResponse {
    pub comments: Vec<TransformedComment>,
}