use super::{TokenManager, ServerInfo};

use actix_web::{web, get, Responder};





// QRコードにするURLのやり取り
#[get("/api/d2m/mobile-url")]
async fn get_d2m_mobile_url(
    token_manager: web::Data<TokenManager>,
    server_info: web::Data<ServerInfo>
) -> impl Responder {
    let token = token_manager.d2m_token.lock().unwrap();


    "aiueo"
}
