pub mod articles;
pub mod technologies;
pub mod entertainment; 

use actix_web::{web, HttpResponse};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(homepage)));
    cfg.service(web::resource("/articles").route(web::get().to(articles::fetch_articles)));
    cfg.service(web::resource("/technologies").route(web::get().to(technologies::fetch_technology_news)));
    cfg.service(web::resource("/entertainment").route(web::get().to(entertainment::display_entertainment)));
}

async fn homepage() -> HttpResponse {
    let html_content = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Blog Home</title>
        <link rel="stylesheet" type="text/css" href="/static/styles.css">
        <link rel="stylesheet"href="https://fonts.googleapis.com/css?family=Tangerine">
    </head>
    <body>
        <header>
            <h1>Chic Chatter</h1>
        </header>
        <div class="container">
        <nav>
            <ul>
                <li><a href="/articles">Articles</a></li>
                <li><a href="/technologies">Technology News</a></li>
                <li><a href="/entertainment">Entertainment News</a></li>
            </ul>
        </nav>
        </div>
    </body>
    </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html_content)
}