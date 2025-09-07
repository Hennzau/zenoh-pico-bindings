extern crate alloc;

use zenoh_protocol::core::ZenohIdProto;

use crate::{
    bindings::{
        z_closure_zid, z_closure_zid_move, z_id_t, z_info_peers_zid, z_info_routers_zid,
        z_info_zid, z_owned_closure_zid_t,
    },
    *,
};

pub struct SessionInfo<'a> {
    session: &'a Session,
}

impl<'a> SessionInfo<'a> {
    pub(crate) fn new(session: &'a Session) -> Self {
        Self { session }
    }

    pub fn zid(&self) -> ZResult<ZenohIdProto> {
        let id = unsafe { z_info_zid(self.session.loan()) };

        ZenohIdProto::try_from(id.id)
    }

    pub fn routers_zid(&self) -> ZResult<alloc::vec::Vec<ZenohIdProto>> {
        use alloc::vec::Vec;

        let mut vec = Vec::new();

        let mut closure = |zid: *const z_id_t| {
            if let Ok(zid) = ZenohIdProto::try_from(unsafe { (*zid).id }) {
                vec.push(zid);
            }
        };

        let (data, callback) = closure.to_ffi();

        let mut closure = core::mem::MaybeUninit::<z_owned_closure_zid_t>::uninit();

        let mut closure = unsafe {
            z_closure_zid(closure.as_mut_ptr(), Some(callback), None, data).to_zerror()?;

            closure.assume_init()
        };

        unsafe {
            z_info_routers_zid(self.session.loan(), z_closure_zid_move(&mut closure)).to_zerror()?
        };

        Ok(vec)
    }

    pub fn peers_zid(&self) -> ZResult<alloc::vec::Vec<ZenohIdProto>> {
        use alloc::vec::Vec;

        let mut vec = Vec::new();

        let mut closure = |zid: *const z_id_t| {
            if let Ok(zid) = ZenohIdProto::try_from(unsafe { (*zid).id }) {
                vec.push(zid);
            }
        };

        let (data, callback) = closure.to_ffi();

        let mut closure = core::mem::MaybeUninit::<z_owned_closure_zid_t>::uninit();

        let mut closure = unsafe {
            z_closure_zid(closure.as_mut_ptr(), Some(callback), None, data).to_zerror()?;

            closure.assume_init()
        };

        unsafe {
            z_info_peers_zid(self.session.loan(), z_closure_zid_move(&mut closure)).to_zerror()?
        };

        Ok(vec)
    }
}
