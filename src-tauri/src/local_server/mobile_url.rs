use super::ActixData;

use actix_web::{web, get, Responder};





// QRコードにするURLのやり取り
#[get("/api/d2m/mobile-url")]
async fn get_d2m_mobile_url(
    data: web::Data<ActixData>,
) -> impl Responder {
    println!("[POST] /api/d2m/mobile-url");

    data.token.clone()
}
