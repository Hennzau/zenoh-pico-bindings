extern crate alloc;

use core::mem::MaybeUninit;
use std::mem::ManuallyDrop;

use crate::{
    ZResult,
    bindings::{
        z_config_default, z_config_drop, z_config_loan_mut, z_config_move, z_loaned_config_t,
        z_moved_config_t, z_owned_config_t, zp_config_insert,
    },
    zerror,
};

pub struct Config {
    pub(crate) inner: z_owned_config_t,
    pub(crate) moved: bool,
}

impl Config {
    pub fn default() -> ZResult<Self> {
        unsafe {
            let mut inner = MaybeUninit::uninit();

            (z_config_default(inner.as_mut_ptr()) == 0)
                .then_some(())
                .ok_or(zerror!("Failed to initialize to default config."))?;

            Ok(Self {
                inner: inner.assume_init(),
                moved: false,
            })
        }
    }

    pub fn with_config(mut self, key: u32, value: &'static str) -> ZResult<Self> {
        unsafe {
            let cstring = alloc::ffi::CString::new(value)?;

            (zp_config_insert(self.loan_mut(), key as u8, cstring.as_ptr()) == 0)
                .then_some(self)
                .ok_or(zerror!("Incorrect (key, value) pair configuration.").into())
        }
    }

    pub(crate) fn loan_mut(&mut self) -> *mut z_loaned_config_t {
        unsafe { z_config_loan_mut(&mut self.inner) }
    }

    pub(crate) fn take(&mut self) -> *mut z_moved_config_t {
        self.moved = true;

        unsafe { z_config_move(&mut self.inner) }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            if !self.moved {
                z_config_drop(z_config_move(&mut self.inner));
            }
        }
    }
}
