use actix_web::{get, web, HttpResponse, Responder, post};
use std::sync::Arc;
use serde::Deserialize;
use crate::application::services::comment_service::CommentService;

#[derive(Deserialize)]
pub struct PathParams {
    pub unique_id: String,
}

#[derive(Deserialize)]
pub struct LotteryRequest {
    pub participants: Vec<String>,
    pub num_winners: usize,
}

#[get("/comments/{unique_id}")]
pub async fn get_comments(
    service: web::Data<Arc<CommentService>>,
    path: web::Path<PathParams>,
) -> impl Responder {
    let unique_id = &path.unique_id;
    match service.get_comments(unique_id).await {
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