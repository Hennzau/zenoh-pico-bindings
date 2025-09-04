#[allow(warnings)]
pub mod bindings;

// pub use bindings::{Z_CONFIG_CONNECT_KEY, Z_CONFIG_LISTEN_KEY, Z_CONFIG_MODE_KEY};

pub(crate) mod config;
pub use config::*;

pub(crate) mod api;
pub use api::*;

pub(crate) mod result;
pub use result::*;

pub fn open(config: Config) -> ZResult<Session> {
    Session::open(config)
}
