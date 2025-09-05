extern crate alloc;

use core::ptr::null_mut;

use crate::{
    bindings::{
        z_loaned_session_t, z_open, z_owned_session_t, z_session_drop, z_session_loan,
        z_session_loan_mut, z_session_move, zp_start_lease_task, zp_start_read_task,
    },
    *,
};

pub struct Session {
    zp: alloc::boxed::Box<z_owned_session_t>,
}

impl Session {
    pub fn info<'a>(&'a self) -> SessionInfo<'a> {
        SessionInfo::new(self)
    }

    pub(crate) fn open(mut config: Config) -> ZResult<Session> {
        let mut session =
            alloc::boxed::Box::new(core::mem::MaybeUninit::<z_owned_session_t>::uninit());

        let mut session = unsafe {
            z_open(session.as_mut_ptr(), config.take(), null_mut()).to_zerror()?;

            Self {
                zp: session.assume_init(),
            }
        };

        session.start_tasks()?; // because session is constructed before, this will drop the session if error happens

        Ok(session)
    }

    pub(crate) fn start_tasks(&mut self) -> ZResult<()> {
        unsafe {
            zp_start_read_task(self.loan_mut(), null_mut()).to_zerror()?;
            zp_start_lease_task(self.loan_mut(), null_mut()).to_zerror()?;
        }

        Ok(())
    }

    pub(crate) fn loan(&self) -> *const z_loaned_session_t {
        unsafe { z_session_loan(self.zp.as_ref()) }
    }

    pub(crate) fn loan_mut(&mut self) -> *mut z_loaned_session_t {
        unsafe { z_session_loan_mut(self.zp.as_mut()) }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            z_session_drop(z_session_move(&mut *self.zp));
        }
    }
}
