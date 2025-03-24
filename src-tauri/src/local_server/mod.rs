mod data;
mod files;
mod filenames;
mod client_pages;

pub use data::{ServerInfo, ActixData, D2MFile, M2DFile, D2MFileName, M2DFileName};
use client_pages::serve_client_pages;
use filenames::{
    get_d2m_filenames,
    get_m2d_filenames,
    post_d2m_filenames,
    post_m2d_filenames,
};
use files::{
    get_d2m_files,
    get_m2d_files,
    post_d2m_files,
    post_m2d_files,
};

use super::dev_configurator::{IS_CLIENT_PAGES_DEV, IS_API_DEV};

use actix_web::{web, App, HttpServer, HttpRequest, get};
use actix_files::Files;
use std::net::TcpListener;
use pnet::datalink;
use tauri::Manager;


#[get("/ip")]
async fn get_ip(req: HttpRequest) -> String {
    let ip = req.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_else(|| "unknown".to_string());
    format!("Your IP address is: {}", ip)
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

    let server_info = ServerInfo::new(api_port, client_pages_port);

    let client_pages_dir = app.path().resource_dir().unwrap().join("client-pages");

    let data = web::Data::new(ActixData::new(server_info.clone(), client_pages_dir.clone()));

    tauri::async_runtime::spawn(async move {
        HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .service(get_ip)
                .service(get_d2m_filenames)
                .service(get_m2d_filenames)
                .service(post_d2m_filenames)
                .service(post_m2d_filenames)
                .service(get_d2m_files)
                .service(get_m2d_files)
                .service(post_d2m_files)
                .service(post_m2d_files)
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
