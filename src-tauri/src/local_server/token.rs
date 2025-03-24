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
use std::time::{Duration, Instant};
use rand::distr::{Alphanumeric, SampleString};


pub struct TokenManager {
    d2m_token: Mutex<(Option<String>, Instant)>,
    m2d_token: Mutex<(Option<String>, Instant)>,
    expiration: Duration,
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

    fn set_d2m_token(&self) {
        self.d2m_token.lock().unwrap().0 = Some(Self::generate_token_string());
        self.d2m_token.lock().unwrap().1 = Instant::now();
    }

    fn set_m2d_token(&self) {
        self.m2d_token.lock().unwrap().0 = Some(Self::generate_token_string());
        self.m2d_token.lock().unwrap().1 = Instant::now();
    }

    fn get_d2m_remaining_secs(&self) -> u64 {
        let token = self.d2m_token.lock().unwrap();
        if token.0.is_none() || token.1.elapsed() > self.expiration {
            0
        } else {
            self.expiration.as_secs() - token.1.elapsed().as_secs()
        }
    }

    fn get_m2d_remaining_secs(&self) -> u64 {
        let token = self.m2d_token.lock().unwrap();
        if token.0.is_none() || token.1.elapsed() > self.expiration {
            0
        } else {
            self.expiration.as_secs() - token.1.elapsed().as_secs()
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
}


