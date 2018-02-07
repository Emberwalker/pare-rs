extern crate futures;
extern crate tokio_core;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod structs;

use hyper::header::Headers;
use structs::*;

// Setup the X-API-Key type for Hyper header handling.
header! { (XAPIKey, "X-API-Key") => [String] }

/// Runs the main routine for shortening.
pub fn main_shorten(
    _conf: &Config,
    _code: Option<String>,
    _meta: Option<String>,
    _url: String,
) -> Result<(), String> {
    // TODO
    Ok(())
}

/// Runs the main routine for deleting.
pub fn main_delete(_conf: &Config, _fail_on_noexist: bool, _code: String) -> Result<(), String> {
    // TODO
    Ok(())
}

/// Runs the main routine for metadata fetching.
pub fn main_meta(_conf: &Config, _json_out: bool, _code: String) -> Result<(), String> {
    // TODO
    Ok(())
}
