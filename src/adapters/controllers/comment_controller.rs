use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;
use serde::Deserialize;
use crate::application::services::comment_service::CommentService;

#[derive(Deserialize)]
struct PathParams {
    unique_id: String,
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