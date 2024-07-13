// mod adapters;
// mod application;
// mod domain;
// mod infrastructure;

// use reqwest;
// use kuchiki::traits::*;
// use std::error::Error;
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // URL de la página web a extraer
//     let url = "https://www.tiktok.com/@canchitatime/video/6966399429838458117";
//
//     // Realizar la solicitud HTTP GET
//     let response = reqwest::get(url).await?.text().await?;
//     println!("{}", response);
//
//     // Analizar el HTML
//     // let document = kuchiki::parse_html().one(response);
//
//     // Definir el selector para los divs con la clase específica
//     // let selector = "div.css-1i7ohvi-DivCommentItemContainer.eo72wou0";
//
//     // Iterar sobre todos los elementos que coincidan con el selector
//     // for css_match in document.select(selector)? {
//     //     let as_node = css_match.as_node();
//     //
//     //     // Extraer y procesar datos del div
//     //     if let Some(id) = as_node
//     //         .select_first("div[class^='css-ulyotp-DivCommentContentContainer']")
//     //         .ok()
//     //         .and_then(|n| n.attributes.borrow().get("id").map(|v| v.to_string()))
//     //     {
//     //         println!("ID: {}", id);
//     //     }
//     //
//     //     // Extraer el nombre de usuario y el comentario
//     //     if let Some(username) = as_node
//     //         .select_first("span[data-e2e='comment-username-1']")
//     //         .ok()
//     //         .and_then(|n| n.text_contents().trim().to_string().into())
//     //     {
//     //         println!("Username: {}", username);
//     //     }
//     //
//     //     if let Some(comment) = as_node
//     //         .select_first("span[dir='']")
//     //         .ok()
//     //         .and_then(|n| n.text_contents().trim().to_string().into())
//     //     {
//     //         println!("Comment: {}", comment);
//     //     }
//     // }
//
//     Ok(())
// }

mod domain;
mod application;
mod infrastructure;
mod adapters;

use actix_web::{web, App, HttpServer};
use adapters::controllers::comment_controller::get_comments;
use infrastructure::config::configure_services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let comment_service = configure_services().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(comment_service.clone()))
            //.app_data(web::Data::new(other_service.clone()))
            .service(get_comments)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

