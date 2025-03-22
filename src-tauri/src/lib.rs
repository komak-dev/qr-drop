mod local_server;
mod dev_configurator;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dbg!(*dev_configurator::IS_CLIENT_PAGES_DEV);
    dbg!(*dev_configurator::IS_API_DEV);

    tauri::Builder::default()
        .setup(|app| {
            let server_info = local_server::start_server(&app);
            let client_pages_url = format!("http://{}:{}", server_info.ip, server_info.client_pages_port);
            let api_url = format!("http://{}:{}", server_info.ip, server_info.api_port);

            println!("\nServer for client pages is running on: {}", client_pages_url);
            println!("Server for api is running on: {}\n", api_url);

            tauri::webview::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External(client_pages_url.parse().unwrap()),
            ).title("QRDrop")
                .resizable(false)
                .inner_size(600.0, 400.0)
                .build()
                .unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
