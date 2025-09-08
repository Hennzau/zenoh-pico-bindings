use core::ptr::null_mut;

use zenoh_buffers::ZBuf;
use zenoh_keyexpr::OwnedKeyExpr;
use zenoh_result::ZResult;

use crate::{bindings::z_put, *};

impl Session {
    pub fn put(&self, keyexpr: &OwnedKeyExpr, payload: impl Into<ZBuf>) -> ZResult<()> {
        let keyexpr = self.declare_keyexpr(keyexpr)?;

        let zbuf: ZBuf = payload.into();
        let mut payload = make_payload(&zbuf)?;

        unsafe {
            z_put(self.loan(), keyexpr.loan(), payload.take(), null_mut()).to_zerror()?;
        };

        Ok(())
    }
}
