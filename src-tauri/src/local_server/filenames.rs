use super::{TFileNames, TokenManager};

use actix_web::{web, get, post, Responder};





// PCやスマホが送ろうとしているファイル名のリストのやり取り
#[get("/api/d2m/filenames")]
pub async fn get_d2m_filenames(data: web::Data<TFileNames>) -> impl Responder {
    println!("[GET] /api/d2m/filenames");

    let filenames = data.d2m.lock().unwrap().clone();
    web::Json(filenames)
}

#[get("/api/m2d/filenames")]
pub async fn get_m2d_filenames(data: web::Data<TFileNames>) -> impl Responder {
    println!("[GET] /api/m2d/filenames");

    let filenames = data.m2d.lock().unwrap().clone();
    web::Json(filenames)
}

#[post("/api/d2m/filenames")]
pub async fn post_d2m_filenames(data: web::Data<TFileNames>, filenames: web::Json<Vec<String>>) -> impl Responder {
    println!("[POST] /api/d2m/filenames");

    *data.d2m.lock().unwrap() = filenames.0.clone();
    web::Json(filenames.0)
}

#[post("/api/m2d/filenames")]
pub async fn post_m2d_filenames(data: web::Data<TFileNames>, filenames: web::Json<Vec<String>>) -> impl Responder {
    println!("[POST] /api/m2d/filenames");

    *data.m2d.lock().unwrap() = filenames.0.clone();
    web::Json(filenames.0)
}


