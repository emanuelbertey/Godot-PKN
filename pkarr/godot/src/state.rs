use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;


pub static VEC_DATA: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));

//use once_cell::sync::Lazy;
//use std::sync::Mutex;

pub static DOWNLOADED_DATA: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static GLOBAL_ARRAY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub static PEER_IPS: Lazy<Mutex<HashMap<String, Vec<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));




pub static GLOBAL_IPS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static GLOBAL_HTTP: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub static IP_IPFS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static ID_IPFS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
