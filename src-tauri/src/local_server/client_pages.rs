use super::ActixData;

use actix_web::{web, get, HttpRequest, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;





// クライアント側のページ(Next.js)を提供する
#[get("/{path:.*}")]
pub async fn serve_client_pages(req: HttpRequest, data: web::Data<ActixData>) -> impl Responder {
    let base_dir = PathBuf::from(&data.client_pages_dir);
    let path: PathBuf = base_dir.join(req.match_info().query("path"));

    println!("[GET] {}", path.display());

    dbg!(&path);

    if path == base_dir {
        NamedFile::open(path.join("index.html"))
    } else if path.extension().is_none() {
        NamedFile::open(path.with_extension("html"))
    } else {
        NamedFile::open(path)
    }
}
