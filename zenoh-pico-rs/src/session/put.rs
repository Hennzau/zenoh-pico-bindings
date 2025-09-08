use core::ptr::null_mut;

use zenoh_keyexpr::OwnedKeyExpr;
use zenoh_result::ZResult;

use crate::{bindings::z_put, *};

impl Session {
    pub fn put(
        &self,
        keyexpr: &OwnedKeyExpr,
        payload: impl TryInto<ZBytes, Error = Error>,
    ) -> ZResult<()> {
        let keyexpr = self.declare_keyexpr(keyexpr)?;
        let mut payload = payload.try_into()?;

        unsafe {
            z_put(self.loan(), keyexpr.loan(), payload.take(), null_mut()).to_zerror()?;
        };

        Ok(())
    }
}
