use std::io;
use std::fs::File;
use std::env;

use serde_json;

#[derive(Debug, Deserialize)]
struct DiskConfig {
    pub apikey: Option<String>,
    pub server: Option<String>,
}

impl DiskConfig {

}

/// A potentially incomplete configuration for Pare.
#[derive(Debug)]
pub struct Config {
    pub apikey: Option<String>,
    pub server: Option<String>,
    pub debug: bool,
}

impl Config {
    /// Returns a version of this Config merged with on-disk configuration.
    pub fn merge_with_disk(self) -> Config {
        match Config::key_server_from_disk() {
            Err(e) => {
                if self.debug { eprintln!("Unable to load disk config: {}", e); }
                Config { ..self }
            }
            Ok((key, srv)) => {
                let mut apikey = self.apikey;
                if None == apikey {
                    apikey = key;
                }
                let mut server = self.server;
                if None == server {
                    server = srv;
                }
                Config {
                    apikey,
                    server,
                    debug: self.debug,
                }
            }
        }
    }

    fn key_server_from_disk() -> Result<(Option<String>, Option<String>), io::Error> {
        let path_opt = env::home_dir();
        if None == path_opt {
            eprintln!("Unable to find home directory.");
            return Ok((None, None));
        }
        let mut path = path_opt.unwrap();
        path.push(".pare.json");

        let json: serde_json::Result<DiskConfig> = serde_json::from_reader(File::open(path)?);
        match json {
            Ok(res) => Ok((res.apikey, res.server)),
            Err(e) => {
                eprintln!("Error parsing ~/.pare.json: {}", e);
                Ok((None, None))
            }
        }
    }
}
