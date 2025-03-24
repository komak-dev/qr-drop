mod files;

use files::TFileNames;

use actix_web::{web, get, App, HttpRequest, HttpServer, Responder};
use actix_files::{NamedFile, Files};
use std::path::PathBuf;
use std::net::TcpListener;
use pnet::datalink;
use tauri::Manager;
use std::sync::Mutex;

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
                .app_data(web::Data::new(TFileNames::new()))
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
