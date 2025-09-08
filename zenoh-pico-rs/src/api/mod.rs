extern crate alloc;

pub(crate) mod info;
pub use info::*;

pub(crate) mod scouting;
pub use scouting::*;

pub(crate) mod keyexpr;
pub use keyexpr::*;

pub(crate) mod zbytes;
pub use zbytes::*;

use zenoh_protocol::core::CowStr;

use crate::{
    bindings::{
        Z_CONFIG_CONNECT_KEY, Z_CONFIG_LISTEN_KEY, Z_CONFIG_MODE_CLIENT, Z_CONFIG_MODE_KEY,
        Z_CONFIG_MODE_PEER, z_config_default, z_config_drop, z_config_loan_mut, z_config_move,
        z_loaned_config_t, z_moved_config_t, z_owned_config_t, zp_config_insert,
    },
    *,
};

pub enum ConfigKey {
    Connect,
    Listen,
    Mode,
}

impl ConfigKey {
    pub(crate) fn to_key(&self) -> u8 {
        match self {
            ConfigKey::Connect => Z_CONFIG_CONNECT_KEY as u8,
            ConfigKey::Listen => Z_CONFIG_LISTEN_KEY as u8,
            ConfigKey::Mode => Z_CONFIG_MODE_KEY as u8,
        }
    }
}

pub enum ValueKey<'a> {
    Endpoint(CowStr<'a>),
    Client,
    Peer,
}

impl ValueKey<'_> {
    pub(crate) fn to_value(&self) -> ZResult<alloc::ffi::CString> {
        match self {
            ValueKey::Endpoint(s) => Ok(alloc::ffi::CString::new(s.as_ref())?),
            ValueKey::Client => Ok(alloc::ffi::CString::new(Z_CONFIG_MODE_CLIENT)?),
            ValueKey::Peer => Ok(alloc::ffi::CString::new(Z_CONFIG_MODE_PEER)?),
        }
    }
}

pub struct Config {
    zp: z_owned_config_t,
    moved: bool,
}

impl Config {
    pub fn default() -> ZResult<Config> {
        let mut config = core::mem::MaybeUninit::<z_owned_config_t>::uninit();

        Ok(unsafe {
            z_config_default(config.as_mut_ptr()).to_zerror()?;

            Self {
                zp: config.assume_init(),
                moved: false,
            }
        })
    }

    pub fn insert(&mut self, key: ConfigKey, value: ValueKey) -> ZResult<()> {
        unsafe {
            zp_config_insert(self.loan_mut(), key.to_key(), value.to_value()?.as_ptr()).to_zerror()
        }
    }

    pub fn with(mut self, key: ConfigKey, value: ValueKey) -> ZResult<Self> {
        self.insert(key, value)?;

        Ok(self)
    }

    pub(crate) fn loan_mut(&mut self) -> *mut z_loaned_config_t {
        unsafe { z_config_loan_mut(&mut self.zp) }
    }

    pub(crate) fn take(&mut self) -> *mut z_moved_config_t {
        if self.moved {
            panic!("Config has already been moved");
        }

        self.moved = true;

        unsafe { z_config_move(&mut self.zp) }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        if !self.moved {
            unsafe {
                z_config_drop(self.take());
            }
        }
    }
}
