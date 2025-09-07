extern crate alloc;

use core::ptr::null_mut;

use zenoh_buffers::{ZBuf, buffer::Buffer};
use zenoh_keyexpr::OwnedKeyExpr;

use crate::{
    bindings::{
        z_bytes_from_buf, z_bytes_move, z_declare_keyexpr, z_keyexpr_loan, z_keyexpr_move,
        z_loaned_session_t, z_open, z_owned_bytes_t, z_owned_keyexpr_t, z_owned_session_t, z_put,
        z_session_drop, z_session_loan, z_session_loan_mut, z_session_move, z_undeclare_keyexpr,
        z_view_keyexpr_from_str, z_view_keyexpr_loan, z_view_keyexpr_t, zp_start_lease_task,
        zp_start_read_task,
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

        session.start_tasks()?;

        Ok(session)
    }

    pub fn put(&self, keyexpr: &OwnedKeyExpr, payload: impl Into<ZBuf>) -> ZResult<()> {
        let zbuf: ZBuf = payload.into();
        let keyexpr = alloc::ffi::CString::new(keyexpr.as_str())?;

        let mut vke = core::mem::MaybeUninit::<z_view_keyexpr_t>::uninit();
        let vke = unsafe {
            z_view_keyexpr_from_str(vke.as_mut_ptr(), keyexpr.as_ptr()).to_zerror()?;

            vke.assume_init()
        };

        let mut ke = core::mem::MaybeUninit::<z_owned_keyexpr_t>::uninit();
        let mut ke = unsafe {
            z_declare_keyexpr(self.loan(), ke.as_mut_ptr(), z_view_keyexpr_loan(&vke))
                .to_zerror()?;

            ke.assume_init()
        };

        let mut payload = core::mem::MaybeUninit::<z_owned_bytes_t>::uninit();
        let mut payload = unsafe {
            z_bytes_from_buf(
                payload.as_mut_ptr(),
                zbuf.to_zslice().as_ptr() as *mut u8,
                zbuf.len(),
                None,
                null_mut(),
            )
            .to_zerror()
            .or_else(|e| {
                z_undeclare_keyexpr(self.loan(), z_keyexpr_move(&mut ke));

                Err(e)
            })?;

            payload.assume_init()
        };

        unsafe {
            z_put(
                self.loan(),
                z_keyexpr_loan(&ke),
                z_bytes_move(&mut payload),
                null_mut(),
            )
            .to_zerror()
            .or_else(|e| {
                z_undeclare_keyexpr(self.loan(), z_keyexpr_move(&mut ke));

                Err(e)
            })?;

            z_undeclare_keyexpr(self.loan(), z_keyexpr_move(&mut ke));
        };

        Ok(())
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
