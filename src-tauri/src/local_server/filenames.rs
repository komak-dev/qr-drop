use super::{ActixData, D2MFileName, M2DFileName};

use actix_web::{web, get, post, Responder, HttpRequest, HttpResponse};
use std::collections::HashMap;





// PCやスマホが送ろうとしているファイル名のリストのやり取り
#[get("/api/d2m/filenames")]
pub async fn get_d2m_filenames(
    data: web::Data<ActixData>, 
    query: web::Query<HashMap<String, String>>
) -> actix_web::Result<HttpResponse> {
    println!("[GET] /api/d2m/filenames");

    if !data.validate_token(query.get("token").unwrap_or(&"".to_string())) {
        return Err(actix_web::error::ErrorUnauthorized("Invalid Token"));
    }

    let filenames = data.d2m_filenames.lock().unwrap().clone();

    Ok(HttpResponse::Ok().json(filenames))
}

#[get("/api/m2d/filenames")]
pub async fn get_m2d_filenames(
    data: web::Data<ActixData>,
    query: web::Query<HashMap<String, String>>
) -> actix_web::Result<HttpResponse> {
    println!("[GET] /api/m2d/filenames");

    if !data.validate_token(query.get("token").unwrap_or(&"".to_string())) {
        return Err(actix_web::error::ErrorUnauthorized("Invalid Token"));
    }

    let filenames = data.m2d_filenames.lock().unwrap().clone();

    Ok(HttpResponse::Ok().json(filenames))
}

#[post("/api/d2m/filenames")]
pub async fn post_d2m_filenames(
    data: web::Data<ActixData>,
    filenames: web::Json<Vec<String>>,
    query: web::Query<HashMap<String, String>>,
) -> actix_web::Result<HttpResponse> {
    println!("[POST] /api/d2m/filenames");

    if !data.validate_token(query.get("token").unwrap_or(&"".to_string())) {
        return Err(actix_web::error::ErrorUnauthorized("Invalid Token"));
    }

    *data.d2m_filenames.lock().unwrap() = filenames.0
        .iter()
        .map(|filename| D2MFileName { name: filename.clone() })
        .collect();

    Ok(HttpResponse::Ok().finish())
}

#[post("/api/m2d/filenames")]
pub async fn post_m2d_filenames(
    data: web::Data<ActixData>,
    filenames: web::Json<Vec<String>>,
    req: HttpRequest,
    query: web::Query<HashMap<String, String>>,
) -> actix_web::Result<impl Responder> {
    println!("[POST] /api/m2d/filenames");

    if !data.validate_token(query.get("token").unwrap_or(&"".to_string())) {
        return Err(actix_web::error::ErrorUnauthorized("Invalid Token"));
    }

    let ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    *data.m2d_filenames.lock().unwrap() = filenames.0
        .iter()
        .map(|filename| M2DFileName { name: filename.clone(), sender_ip: ip.clone() })
        .collect();

    Ok(HttpResponse::Ok().finish())
}
