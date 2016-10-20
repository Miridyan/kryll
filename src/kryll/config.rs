use super::*;
use toml::*;

use std::fs::File;

#[derive(Debug, RustcDecodable)]
pub struct Config {
    site: Option<SiteConfig>,
    directories: Option<DirConfig>,
}

#[derive(Debug, RustcDecodable)]
struct SiteConfig {
    name: Option<String>,
    template: Option<String>,
    author: Option<Vec<String>>, 
}

#[derive(Debug, RustcDecodable)]
struct DirConfig {
    out: Option<String>,
    source: Option<String>,
    images: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        let config_file = File::open("kryll.toml");
        let config_str = handle_file_string(config_file, "kryll.toml");
        
        let config_decode: Config = decode_str(&config_str).unwrap();
        config_decode
    }
}
