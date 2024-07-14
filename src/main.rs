mod domain;
mod application;
mod infrastructure;
mod adapters;

use actix_web::{web, App, HttpServer};
use adapters::controllers::comment_controller::get_comments;
use infrastructure::config::configure_services;
use crate::adapters::controllers::comment_controller::draw_lottery;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let comment_service = configure_services().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(comment_service.clone()))
            .service(get_comments)
            .service(draw_lottery)
    })
        .bind("0.0.0.0:9080")?
        .run()
        .await
}

