use actix_web::{web, HttpResponse, Responder, post};
use std::sync::Arc;
use serde::Deserialize;
use crate::application::services::comment_service::CommentService;

#[derive(Deserialize)]
pub struct CommentRequest {
    pub url: String,
}

#[derive(Deserialize)]
pub struct LotteryRequest {
    pub participants: Vec<String>,
    pub num_winners: usize,
}

#[post("/comments")]
pub async fn get_comments(
    service: web::Data<Arc<CommentService>>,
    body: web::Json<CommentRequest>,
) -> impl Responder {
    let url = body.url.clone();
    match service.get_comments(&*url).await {
        Ok(transformed_response) => HttpResponse::Ok().json(transformed_response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching comments: {}", e)),
    }
}

#[post("/lottery")]
pub async fn draw_lottery(
    service: web::Data<Arc<CommentService>>,
    body: web::Json<LotteryRequest>,
) -> impl Responder {
    let participants = body.participants.clone();
    let num_winners = body.num_winners.clone();

    match service.get_lottery(participants, num_winners).await {
        Ok(lottery_response) => HttpResponse::Ok().json(lottery_response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error lottery generate: {}", e)),
    }
}