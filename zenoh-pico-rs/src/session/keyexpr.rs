use crate::{
    bindings::{
        z_declare_keyexpr, z_owned_keyexpr_t, z_undeclare_keyexpr, z_view_keyexpr_from_str,
        z_view_keyexpr_loan, z_view_keyexpr_t,
    },
    *,
};

impl Session {
    pub fn declare_keyexpr<'a>(&'a self, keyexpr: &OwnedKeyExpr) -> ZResult<WiredKeyExpr<'a>> {
        let ffi_keyexpr = alloc::ffi::CString::new(keyexpr.as_str())?;

        let mut zp = core::mem::MaybeUninit::<z_owned_keyexpr_t>::uninit();
        let zp = unsafe {
            let mut vke = core::mem::MaybeUninit::<z_view_keyexpr_t>::uninit();
            let vke = {
                z_view_keyexpr_from_str(vke.as_mut_ptr(), ffi_keyexpr.as_ptr()).to_zerror()?;

                vke.assume_init()
            };

            z_declare_keyexpr(self.loan(), zp.as_mut_ptr(), z_view_keyexpr_loan(&vke))
                .to_zerror()?;

            zp.assume_init()
        };

        Ok(WiredKeyExpr::new(keyexpr.clone(), zp, self))
    }

    pub fn undeclare_keyexpr(&self, mut keyexpr: WiredKeyExpr) {
        unsafe { z_undeclare_keyexpr(self.loan(), keyexpr.take()) };
    }
}
