use crate::{
    bindings::{
        z_keyexpr_loan, z_keyexpr_move, z_loaned_keyexpr_t, z_moved_keyexpr_t, z_owned_keyexpr_t,
        z_undeclare_keyexpr,
    },
    *,
};

#[derive(Clone)]
pub struct WiredKeyExpr<'a> {
    keyexpr: OwnedKeyExpr,
    zp: z_owned_keyexpr_t,
    session: &'a Session,
    moved: bool,
}

impl<'a> WiredKeyExpr<'a> {
    pub(crate) fn new(keyexpr: OwnedKeyExpr, zp: z_owned_keyexpr_t, session: &'a Session) -> Self {
        Self {
            keyexpr,
            zp,
            session,
            moved: false,
        }
    }
}

impl WiredKeyExpr<'_> {
    pub fn into_owned(self) -> OwnedKeyExpr {
        self.keyexpr.clone()
    }

    pub(crate) fn loan(&self) -> *const z_loaned_keyexpr_t {
        unsafe { z_keyexpr_loan(&self.zp) }
    }

    pub(crate) fn take(&mut self) -> *mut z_moved_keyexpr_t {
        if self.moved {
            panic!("KeyExpr already moved");
        }

        self.moved = true;

        unsafe { z_keyexpr_move(&mut self.zp) }
    }
}

impl Drop for WiredKeyExpr<'_> {
    fn drop(&mut self) {
        if !self.moved {
            unsafe { z_undeclare_keyexpr(self.session.loan(), self.take()) };
        }
    }
}
