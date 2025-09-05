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
        use alloc::{boxed::Box, vec::Vec};

        let vec = Box::new(Vec::new());
        let raw_ptr: *mut Vec<ZenohIdProto> = Box::into_raw(vec);

        unsafe extern "C" fn router_zid_closure(zid: *const z_id_t, args: *mut core::ffi::c_void) {
            if args.is_null() || zid.is_null() {
                return;
            }

            let vec = args as *mut Vec<ZenohIdProto>;

            unsafe {
                if let Ok(id) = ZenohIdProto::try_from((*zid).id) {
                    (*vec).push(id);
                }
            }
        }

        let mut closure = core::mem::MaybeUninit::<z_owned_closure_zid_t>::uninit();

        let mut closure = unsafe {
            z_closure_zid(
                closure.as_mut_ptr(),
                Some(router_zid_closure),
                None,
                raw_ptr as *mut _,
            )
            .to_zerror()?;

            closure.assume_init()
        };

        unsafe {
            z_info_routers_zid(self.session.loan(), z_closure_zid_move(&mut closure)).to_zerror()?
        };

        Ok(unsafe { *Box::from_raw(raw_ptr) })
    }

    pub fn peers_zid(&self) -> ZResult<alloc::vec::Vec<ZenohIdProto>> {
        use alloc::{boxed::Box, vec::Vec};

        let vec = Box::new(Vec::new());
        let raw_ptr: *mut Vec<ZenohIdProto> = Box::into_raw(vec);

        unsafe extern "C" fn peer_zid_closure(zid: *const z_id_t, args: *mut core::ffi::c_void) {
            if args.is_null() || zid.is_null() {
                return;
            }

            let vec = args as *mut Vec<ZenohIdProto>;

            unsafe {
                if let Ok(id) = ZenohIdProto::try_from((*zid).id) {
                    (*vec).push(id);
                }
            }
        }

        let mut closure = core::mem::MaybeUninit::<z_owned_closure_zid_t>::uninit();

        let mut closure = unsafe {
            z_closure_zid(
                closure.as_mut_ptr(),
                Some(peer_zid_closure),
                None,
                raw_ptr as *mut _,
            )
            .to_zerror()?;

            closure.assume_init()
        };

        unsafe {
            z_info_peers_zid(self.session.loan(), z_closure_zid_move(&mut closure)).to_zerror()?
        };

        Ok(unsafe { *Box::from_raw(raw_ptr) })
    }
}
