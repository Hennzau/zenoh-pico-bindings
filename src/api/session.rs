extern crate alloc;

use core::mem::MaybeUninit;
use std::ptr::null_mut;

use crate::{
    Config, SessionInfo, ZResult,
    bindings::{
        z_loaned_session_t, z_open, z_open_options_t, z_owned_session_t, z_session_drop,
        z_session_loan, z_session_loan_mut, z_session_move, zp_start_lease_task,
        zp_start_read_task, zp_task_lease_options_t, zp_task_read_options_t,
    },
    zerror,
};

pub struct Session {
    pub(crate) inner: z_owned_session_t,
}

impl Session {
    pub(crate) fn open(mut config: Config) -> ZResult<Self> {
        unsafe {
            let mut inner = core::mem::MaybeUninit::<z_owned_session_t>::uninit();

            (z_open(inner.as_mut_ptr(), config.take(), null_mut()) == 0)
                .then_some(())
                .ok_or(zerror!("Unable to open session"))?;

            (zp_start_read_task(z_session_loan_mut(inner.as_mut_ptr()), null_mut()) == 0
                && zp_start_lease_task(z_session_loan_mut(inner.as_mut_ptr()), null_mut()) == 0)
                .then_some(Self {
                    inner: inner.assume_init(),
                })
                .ok_or_else(|| {
                    z_session_drop(z_session_move(inner.as_mut_ptr()));

                    zerror!("Unable to start tasks").into()
                })
        }
    }

    pub fn loan(&self) -> *const z_loaned_session_t {
        unsafe { z_session_loan(&self.inner) }
    }

    pub fn info<'a>(&'a self) -> SessionInfo<'a> {
        SessionInfo { session: self }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            z_session_drop(z_session_move(&mut self.inner));
        }
    }
}
