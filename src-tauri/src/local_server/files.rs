use actix_web::{web, get, post, HttpResponse, Responder};
use actix_multipart::Multipart;
use std::sync::Mutex;
use futures_util::StreamExt as _;
use bytes::{Bytes, BytesMut};
use std::io::Cursor;
use std::io::Write;
use zip::write::SimpleFileOptions;


pub struct TFileNames {
    d2m: Mutex<Vec<String>>,
    m2d: Mutex<Vec<String>>,
}

impl TFileNames {
    pub fn new() -> Self {
        TFileNames {
            d2m: Mutex::new(Vec::new()),
            m2d: Mutex::new(Vec::new()),
        }
    }
}


pub struct TFiles {
    d2m: Mutex<Vec<(String, Bytes)>>,
    m2d: Mutex<Vec<(String, Bytes)>>,
}

impl TFiles {
    pub fn new() -> Self {
        TFiles {
            d2m: Mutex::new(Vec::new()),
            m2d: Mutex::new(Vec::new()),
        }
    }
}



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



// 一時的にメモリ上に保存したファイルのやり取り
#[get("/api/d2m/files")]
pub async fn get_d2m_files(data: web::Data<TFiles>) -> actix_web::Result<HttpResponse> {
    println!("[GET] /api/d2m/files");

    let files = data.d2m.lock().unwrap();

    let mut zip_buf = Cursor::new(Vec::new());
    let mut zip_writer = zip::ZipWriter::new(&mut zip_buf);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for (filename, data) in files.iter() {
        if zip_writer.start_file(filename, options).is_err() {
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
pub async fn get_m2d_files(data: web::Data<TFiles>) -> actix_web::Result<HttpResponse> {
    println!("[GET] /api/m2d/files");

    let files = data.m2d.lock().unwrap();

    let mut zip_buf = Cursor::new(Vec::new());
    let mut zip_writer = zip::ZipWriter::new(&mut zip_buf);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for (filename, data) in files.iter() {
        if zip_writer.start_file(filename, options).is_err() {
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
pub async fn post_d2m_files(data: web::Data<TFiles>, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    println!("[POST] /api/d2m/files");

    let mut files = data.d2m.lock().unwrap();
    files.clear();

    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap_or("unnamed").to_string();
        let mut file_bytes = BytesMut::new();

        while let Some(chunk) = field.next().await {
            file_bytes.extend_from_slice(&chunk?);
        }

        files.push((filename, file_bytes.freeze()));
    }

    Ok(HttpResponse::Ok().finish())
}

#[post("/api/m2d/files")]
pub async fn post_m2d_files(data: web::Data<TFiles>, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    println!("[POST] /api/m2d/files");

    let mut files = data.m2d.lock().unwrap();
    files.clear();

    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap_or("unnamed").to_string();
        let mut file_bytes = BytesMut::new();

        while let Some(chunk) = field.next().await {
            file_bytes.extend_from_slice(&chunk?);
        }

        files.push((filename, file_bytes.freeze()));
    }

    Ok(HttpResponse::Ok().finish())
}
