use super::{ActixData, D2MFile, M2DFile};

use actix_web::{web, get, post, HttpResponse, HttpRequest};
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use bytes::BytesMut;
use std::io::Cursor;
use std::io::Write;
use zip::write::SimpleFileOptions;
use std::collections::HashMap;





// 一時的にメモリ上に保存したファイルのやり取り
#[get("/api/d2m/files")]
pub async fn get_d2m_files(data: web::Data<ActixData>) -> actix_web::Result<HttpResponse> {
    println!("[GET] /api/d2m/files");

    let files = data.d2m_files.lock().unwrap();

    let mut zip_buf = Cursor::new(Vec::new());
    let mut zip_writer = zip::ZipWriter::new(&mut zip_buf);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for D2MFile { name, data } in files.iter() {
        if zip_writer.start_file(name, options).is_err() {
            return Err(actix_web::error::ErrorInternalServerError("Failed to start zip file"));
        }
        zip_writer.write_all(data)?;
    }
    if zip_writer.finish().is_err() {
        return Err(actix_web::error::ErrorInternalServerError("Failed to finish zip file"));
    }

    Ok(HttpResponse::Ok()
        .content_type("application/zip")
        .append_header(("Content-Disposition", "attachment; filename=\"QRDrop.zip\""))
        .body(zip_buf.into_inner()))
}

#[get("/api/m2d/files")]
pub async fn get_m2d_files(data: web::Data<ActixData>, query: web::Query<HashMap<String, String>>) -> actix_web::Result<HttpResponse> {
    println!("[GET] /api/m2d/files");

    let Some(sender_ip) = query.get("senderIp") else {
        return Err(actix_web::error::ErrorBadRequest("senderIp is required"));
    };

    let files = data.m2d_files.lock().unwrap();

    let mut zip_buf = Cursor::new(Vec::new());
    let mut zip_writer = zip::ZipWriter::new(&mut zip_buf);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);


    for M2DFile { name, data, sender_ip: _ } in files.iter().filter(|f| f.sender_ip == *sender_ip) { 
        if zip_writer.start_file(name, options).is_err() {
            return Err(actix_web::error::ErrorInternalServerError("Failed to start zip file"));
        }
        zip_writer.write_all(data)?;
    }
    if zip_writer.finish().is_err() {
        return Err(actix_web::error::ErrorInternalServerError("Failed to finish zip file"));
    }

    Ok(HttpResponse::Ok()
        .content_type("application/zip")
        .append_header(("Content-Disposition", "attachment; filename=\"QRDrop.zip\""))
        .body(zip_buf.into_inner()))
}

#[post("/api/d2m/files")]
pub async fn post_d2m_files(data: web::Data<ActixData>, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    println!("[POST] /api/d2m/files");

    let mut files = data.d2m_files.lock().unwrap();
    files.clear();

    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap_or("unnamed").to_string();
        let mut file_bytes = BytesMut::new();

        while let Some(chunk) = field.next().await {
            file_bytes.extend_from_slice(&chunk?);
        }

        files.push(D2MFile { name: filename, data: file_bytes.freeze() });
    }

    Ok(HttpResponse::Ok().finish())
}

#[post("/api/m2d/files")]
pub async fn post_m2d_files(
    data: web::Data<ActixData>,
    mut payload: Multipart,
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    println!("[POST] /api/m2d/files");

    let mut files = data.m2d_files.lock().unwrap();
    files.clear();

    let sender_ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap_or("unnamed").to_string();
        let mut file_bytes = BytesMut::new();

        while let Some(chunk) = field.next().await {
            file_bytes.extend_from_slice(&chunk?);
        }

        files.push(M2DFile { name: filename, data: file_bytes.freeze(), sender_ip: sender_ip.clone() });
    }

    Ok(HttpResponse::Ok().finish())
}


