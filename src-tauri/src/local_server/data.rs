use std::path::PathBuf;
use std::sync::Mutex;
use bytes::Bytes;
use rand::distr::{Alphanumeric, SampleString};
use pnet::datalink;
use serde::Serialize;





#[derive(Clone, Debug)]
pub struct ServerInfo {
    pub ip: String,
    pub api_port: u16,
    pub client_pages_port: u16,
    pub token: String,
}

impl ServerInfo {
    pub fn new(api_port: u16, client_pages_port: u16) -> Self {
        let ip = datalink::interfaces()
            .iter()
            .filter(|iface| iface.is_up() && !iface.ips.is_empty())
            .flat_map(|iface| iface.ips.iter())
            .filter(|ip| ip.is_ipv4())
            .map(|ip| ip.ip().to_string())
            .collect::<Vec<String>>()[1]
            .clone();

        Self { 
            ip,
            api_port, 
            client_pages_port,
            token: Alphanumeric.sample_string(&mut rand::rng(), 32).to_string(),
        }
    }
}





#[derive(Clone, Debug, Serialize)]
pub struct D2MFileName {
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M2DFileName {
    pub name: String,
    pub sender_ip: String,
}

#[derive(Clone)]
pub struct D2MFile {
    pub name: String,
    pub data: Bytes,
}

#[derive(Clone)]
pub struct M2DFile {
    pub name: String,
    pub data: Bytes,
    pub sender_ip: String,
}


pub struct ActixData {
    pub server_info: ServerInfo,
    pub client_pages_dir: PathBuf,
    pub d2m_filenames: Mutex<Vec<D2MFileName>>,
    pub m2d_filenames: Mutex<Vec<M2DFileName>>,
    pub d2m_files: Mutex<Vec<D2MFile>>,
    pub m2d_files: Mutex<Vec<M2DFile>>,
}


impl ActixData {
    pub fn new(server_info: ServerInfo, client_pages_dir: PathBuf) -> Self {
        Self {
            server_info,
            client_pages_dir,
            d2m_filenames: Mutex::new(Vec::new()),
            m2d_filenames: Mutex::new(Vec::new()),
            d2m_files: Mutex::new(Vec::new()),
            m2d_files: Mutex::new(Vec::new()),
        }
    }

    pub fn validate_token(&self, token: &str) -> bool {
        self.server_info.token == token
    }
}
