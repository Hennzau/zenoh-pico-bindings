#![no_std]

#[allow(warnings)]
pub(crate) mod bindings;

pub(crate) mod result;
pub use result::*;

pub(crate) mod api;
pub use api::*;

pub(crate) mod session;
pub use session::*;

pub mod protocol {
    pub use zenoh_protocol::core::WhatAmI;
}

pub fn open(config: Config) -> ZResult<Session> {
    Session::open(config)
}
