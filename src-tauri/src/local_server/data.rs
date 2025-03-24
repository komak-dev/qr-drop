use std::path::PathBuf;
use std::sync::Mutex;
use bytes::Bytes;
use std::time::{Duration, Instant};
use rand::distr::{Alphanumeric, SampleString};
use pnet::datalink;





#[derive(Clone)]
pub struct ServerInfo {
    pub ip: String,
    pub api_port: u16,
    pub client_pages_port: u16,
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

        Self { ip, api_port, client_pages_port }
    }
}


pub struct StaticFilesManager {
    pub client_pages_dir: PathBuf,
}

impl StaticFilesManager {
    pub fn new(client_pages_dir: PathBuf) -> Self {
        Self { client_pages_dir }
    }
}


pub struct TFileNames {
    pub d2m: Mutex<Vec<String>>,
    pub m2d: Mutex<Vec<String>>,
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
    pub d2m: Mutex<Vec<(String, Bytes)>>,
    pub m2d: Mutex<Vec<(String, Bytes)>>,
}

impl TFiles {
    pub fn new() -> Self {
        TFiles {
            d2m: Mutex::new(Vec::new()),
            m2d: Mutex::new(Vec::new()),
        }
    }
}

pub struct TokenManager {
    pub d2m_token: Mutex<(Option<String>, Instant)>,
    pub m2d_token: Mutex<(Option<String>, Instant)>,
    pub expiration: Duration,
}

impl TokenManager {
    pub fn new() -> Self {
        TokenManager {
            d2m_token: Mutex::new((None, Instant::now())),
            m2d_token: Mutex::new((None, Instant::now())),
            expiration: Duration::from_secs(10 * 60),
        }
    }

    fn generate_token_string() -> String {
        Alphanumeric.sample_string(&mut rand::rng(), 32)
    }

    fn reset_d2m_token(&self) {
        self.d2m_token.lock().unwrap().0 = Some(Self::generate_token_string());
        self.d2m_token.lock().unwrap().1 = Instant::now();
    }

    fn reset_m2d_token(&self) {
        self.m2d_token.lock().unwrap().0 = Some(Self::generate_token_string());
        self.m2d_token.lock().unwrap().1 = Instant::now();
    }

    fn clear_d2m_token(&self) {
        self.d2m_token.lock().unwrap().0 = None;
    }

    fn clear_m2d_token(&self) {
        self.m2d_token.lock().unwrap().0 = None;
    }

    fn get_d2m_remaining_ms(&self) -> u128 {
        let token = self.d2m_token.lock().unwrap();
        if token.0.is_none() || token.1.elapsed() > self.expiration {
            0
        } else {
            self.expiration.as_millis() - token.1.elapsed().as_millis()
        }
    }

    fn get_m2d_remaining_ms(&self) -> u128 {
        let token = self.m2d_token.lock().unwrap();
        if token.0.is_none() || token.1.elapsed() > self.expiration {
            0
        } else {
            self.expiration.as_millis() - token.1.elapsed().as_millis()
        }
    }

    fn validate_d2m_token(&self, t: &str) -> bool {
        let token = self.d2m_token.lock().unwrap();
        token.0.is_some() && token.0.as_ref().unwrap() == t
    }

    fn validate_m2d_token(&self, t: &str) -> bool {
        let token = self.m2d_token.lock().unwrap();
        token.0.is_some() && token.0.as_ref().unwrap() == t
    }

    fn d2m_expired(&self) -> bool {
        self.get_d2m_remaining_ms() == 0
    }

    fn m2d_expired(&self) -> bool {
        self.get_m2d_remaining_ms() == 0
    }
}


