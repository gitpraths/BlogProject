use actix_files::NamedFile;
use actix_web::Result;
pub async fn index() -> Result<NamedFile> {
    NamedFile::open_async("./static/index.html")
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
}