#![no_std]
extern crate alloc;

#[allow(warnings)]
pub mod bindings;

pub(crate) mod result;
pub use result::*;

pub(crate) mod api;
pub use api::*;

pub(crate) mod session;
pub use session::*;

pub(crate) mod utils;
pub(crate) use utils::*;

pub use zenoh_keyexpr::OwnedKeyExpr;
pub use zenoh_protocol::core::{Locator, WhatAmI, WhatAmIMatcher, ZenohIdProto};

pub fn open(config: Config) -> ZResult<Session> {
    Session::open(config)
}

pub fn scout<I: Into<WhatAmIMatcher>>(
    config: Config,
    user_closure: impl FnMut(&ZenohIdProto, &WhatAmI, &alloc::vec::Vec<Locator>),
    options: Option<ScoutOptions<I>>,
) -> ZResult<()> {
    api::scouting::start_scout(config, user_closure, options)
}
