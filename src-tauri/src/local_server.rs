use actix_web::{web, get, post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files::{NamedFile, Files};
use actix_multipart::Multipart;
use std::path::PathBuf;
use std::net::TcpListener;
use pnet::datalink;
use tauri::Manager;
use std::sync::Mutex;
use futures_util::StreamExt as _;
use bytes::{Bytes, BytesMut};
use std::io::Cursor;
use std::io::Write;
use zip::write::SimpleFileOptions;

use super::dev_configurator::{IS_CLIENT_PAGES_DEV, IS_API_DEV};



#[derive(Clone)]
pub struct ServerInfo {
    pub ip: String,
    pub api_port: u16,
    pub client_pages_port: u16,
}

struct StaticFilesManager {
    client_pages_dir: PathBuf,
}


struct TFileNames {
    d2m: Mutex<Vec<String>>,
    m2d: Mutex<Vec<String>>,
}

struct TFiles {
    d2m: Mutex<Vec<(String, Bytes)>>,
    m2d: Mutex<Vec<(String, Bytes)>>,
}



// PCやスマホが送ろうとしているファイル名のリストのやり取り
#[get("/api/d2m/filenames")]
async fn get_d2m_filenames(data: web::Data<TFileNames>) -> impl Responder {
    println!("[GET] /api/d2m/filenames");

    let filenames = data.d2m.lock().unwrap().clone();
    web::Json(filenames)
}

#[get("/api/m2d/filenames")]
async fn get_m2d_filenames(data: web::Data<TFileNames>) -> impl Responder {
    println!("[GET] /api/m2d/filenames");

    let filenames = data.m2d.lock().unwrap().clone();
    web::Json(filenames)
}

#[post("/api/d2m/filenames")]
async fn post_d2m_filenames(data: web::Data<TFileNames>, filenames: web::Json<Vec<String>>) -> impl Responder {
    println!("[POST] /api/d2m/filenames");

    *data.d2m.lock().unwrap() = filenames.0.clone();
    web::Json(filenames.0)
}

#[post("/api/m2d/filenames")]
async fn post_m2d_filenames(data: web::Data<TFileNames>, filenames: web::Json<Vec<String>>) -> impl Responder {
    println!("[POST] /api/m2d/filenames");

    *data.m2d.lock().unwrap() = filenames.0.clone();
    web::Json(filenames.0)
}



// 一時的にメモリ上に保存したファイルのやり取り
#[get("/api/d2m/files")]
async fn get_d2m_files(data: web::Data<TFiles>) -> actix_web::Result<HttpResponse> {
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
async fn get_m2d_files(data: web::Data<TFiles>) -> actix_web::Result<HttpResponse> {
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
async fn post_d2m_files(data: web::Data<TFiles>, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
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
async fn post_m2d_files(data: web::Data<TFiles>, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
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




// クライアント側のページ(Next.js)を提供する
#[get("/{path:.*}")]
async fn serve_client_pages(req: HttpRequest, data: web::Data<StaticFilesManager>) -> impl Responder {
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



// アプリの初期化時に呼び出され、サーバーを起動する
pub fn start_server(app: &tauri::App) -> ServerInfo {
    let ip = datalink::interfaces()
        .iter()
        .filter(|iface| iface.is_up() && !iface.ips.is_empty())
        .flat_map(|iface| iface.ips.iter())
        .filter(|ip| ip.is_ipv4())
        .map(|ip| ip.ip().to_string())
        .collect::<Vec<String>>()[1]
        .clone();

    let listner = if *IS_API_DEV {
        TcpListener::bind("0.0.0.0:8080").unwrap()
    } else {
        TcpListener::bind("0.0.0.0:0").unwrap()
    };

    let api_port = listner.local_addr().unwrap().port();
    let client_pages_port = if *IS_CLIENT_PAGES_DEV { 3000 } else { api_port };

    let server_info = ServerInfo { ip, api_port, client_pages_port };
    let server_info_clone = server_info.clone();

    let client_pages_dir = app.path().resource_dir().unwrap().join("client-pages");

    tauri::async_runtime::spawn(async move {
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(StaticFilesManager {
                    client_pages_dir: client_pages_dir.clone() 
                }))
                .app_data(web::Data::new(server_info_clone.clone()))
                .app_data(web::Data::new(TFileNames {
                    d2m: Mutex::new(Vec::new()),
                    m2d: Mutex::new(Vec::new()),
                }))
                .service(serve_client_pages)
                .service(Files::new("/", &client_pages_dir).index_file("index.html"))
        })
        .listen(listner)
        .unwrap()
        .run()
        .await
        .unwrap();
    });
    
    server_info
}
