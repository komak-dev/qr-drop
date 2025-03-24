use super::{ActixData, D2MFileName, M2DFileName};

use actix_web::{web, get, post, Responder, HttpRequest};





// PCやスマホが送ろうとしているファイル名のリストのやり取り
#[get("/api/d2m/filenames")]
pub async fn get_d2m_filenames(data: web::Data<ActixData>) -> impl Responder {
    println!("[GET] /api/d2m/filenames");

    let filenames = data.d2m_filenames.lock().unwrap().clone();

    web::Json(filenames)
}

#[get("/api/m2d/filenames")]
pub async fn get_m2d_filenames(data: web::Data<ActixData>) -> impl Responder {
    println!("[GET] /api/m2d/filenames");

    dbg!(&data.m2d_filenames.lock().unwrap());

    let filenames = data.m2d_filenames.lock().unwrap().clone();

    web::Json(filenames)
}

#[post("/api/d2m/filenames")]
pub async fn post_d2m_filenames(
    data: web::Data<ActixData>,
    filenames: web::Json<Vec<String>>
) -> impl Responder {
    println!("[POST] /api/d2m/filenames");

    *data.d2m_filenames.lock().unwrap() = filenames.0
        .iter()
        .map(|filename| D2MFileName { name: filename.clone() })
        .collect();

    "OK"
}

#[post("/api/m2d/filenames")]
pub async fn post_m2d_filenames(
    data: web::Data<ActixData>,
    filenames: web::Json<Vec<String>>,
    req: HttpRequest,
) -> impl Responder {
    println!("[POST] /api/m2d/filenames");

    dbg!(&filenames.0);

    let ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    *data.m2d_filenames.lock().unwrap() = filenames.0
        .iter()
        .map(|filename| M2DFileName { name: filename.clone(), sender_ip: ip.clone() })
        .collect();

    dbg!(&data.m2d_filenames.lock().unwrap());

    "OK"
}


