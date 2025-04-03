#![recursion_limit = "1024"]

extern crate base64;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde;
pub extern crate rouille;
#[cfg(test)]
extern crate serde_bytes;
extern crate serde_xml_rs;
extern crate xml;
pub extern crate reqwest;

pub mod client;
pub mod error;
pub mod server;
mod xmlfmt;

pub use crate::client::{call, call_value, Client};
pub use crate::server::Server;
pub use crate::xmlfmt::{from_params, into_params, Call, Fault, Params, Response, Value};
