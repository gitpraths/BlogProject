use actix_web::{HttpResponse, Responder};
use reqwest::Client;
use crate::models::{ApiResponse, Article};

const API_KEY: &str = "72854e3253334c7f8b3812eb93422a58"; 
const USER_AGENT: &str = "BLOG_ENGINE/1.0"; 


async fn fetch_from_newsapi() -> Result<Vec<Article>, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://newsapi.org/v2/top-headlines?country=us&apiKey={}",
        API_KEY
    );

    let response = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;
        
    Ok(response.articles)
}

pub async fn fetch_articles() -> impl Responder {
    match fetch_from_newsapi().await {
        Ok(articles) => {
          let mut html_content = String::from(r##"
          <!DOCTYPE html>
          <html lang="en">
          <head>
              <meta charset="UTF-8">
              <title>Articles</title>
              <link href="//netdna.bootstrapcdn.com/twitter-bootstrap/2.3.2/css/bootstrap-combined.no-icons.min.css" rel="stylesheet">
              <link href="//netdna.bootstrapcdn.com/font-awesome/3.2.1/css/font-awesome.css" rel="stylesheet">
              <link rel="stylesheet" type="text/css" href="/static/styles.css">
          </head>
          <body>
              <a href="/" aria-label="Return to home">
                  <svg width="30" height="30" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M15 18l-6-6 6-6" stroke="#000" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
              </a>
              <header>
                  <h1 id="sections">Articles</h1>
                  </header>
      "##); 

            for article in articles {
                let title = format_markdown(&article.title);
                let description = article.description.clone().unwrap_or_default();
                let author = article.author.clone().unwrap_or_else(|| "Unknown author".to_string());
                let published_at = &article.published_at;
                let source_name = &article.source.name;
                let image_url = article.url_to_image.clone().unwrap_or_else(|| "default_image_url.jpg".to_string());

                html_content.push_str(&format!(
                    "<h2>{}</h2><p><strong>By:</strong> {} | <strong>Source:</strong> {} | <strong>Published At:</strong> {}</p><p>{}</p><a href=\"{}\">Read more</a><br><img src=\"{}\" alt=\"Image\">",
                    title, author, source_name, published_at, description, article.url, image_url
                ));
            }

            html_content.push_str("</body></html>");
            HttpResponse::Ok().content_type("text/html").body(html_content)
        },
        Err(err) => {
            eprintln!("Error fetching articles: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Failed to fetch articles: {:?}", err))
        }
    }
}

fn format_markdown(input: &str) -> String {
    use pulldown_cmark::{Parser, html};

    let parser = Parser::new(input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}