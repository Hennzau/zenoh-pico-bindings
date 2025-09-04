extern crate alloc;

use alloc::vec::Vec;
use zenoh_result::zerror;

use core::ffi::c_void;
use std::ops::Neg;

use zenoh_protocol::core::ZenohIdProto;

use crate::{
    Session, ZResult,
    bindings::{
        z_closure_zid, z_closure_zid_drop, z_closure_zid_move, z_free, z_id_t, z_info_routers_zid,
        z_info_zid, z_owned_closure_zid_t,
    },
};

pub struct SessionInfo<'a> {
    pub(crate) session: &'a Session,
}

impl<'a> SessionInfo<'a> {
    pub fn zid(&'a self) -> ZResult<ZenohIdProto> {
        unsafe {
            let zid = z_info_zid(self.session.loan());
            // ZenohIdProto::try_from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
            ZenohIdProto::try_from(zid.id)
        }
    }

    pub fn routers_zid(&'a self) -> ZResult<impl Iterator<Item = ZenohIdProto>> {
        unsafe {
            // let mut res: Vec<ZenohIdProto> = Vec::with_capacity(123);

            // unsafe extern "C" fn routers_callback(zid: *const z_id_t, args: *mut c_void) {
            //     println!("iauhzdohada");
            //     if zid.is_null() || args.is_null() {
            //         return;
            //     }

            //     unsafe {
            //         // let res: &mut Vec<ZenohIdProto> = &mut *(args as *mut Vec<ZenohIdProto>);

            //         // let zid_value = *zid;

            //         // if let Ok(zid) = ZenohIdProto::try_from(zid_value.id) {
            //         //     res.push(zid);
            //         // }
            //     }
            // }

            // let mut callback = core::mem::zeroed::<z_owned_closure_zid_t>();
            // let mut args = core::mem::zeroed::<c_void>();

            // (z_closure_zid(&mut callback, None, None, &mut args) == 0)
            //     .then_some(())
            //     .ok_or(zerror!("Unable to create closure for zid"))?;

            // z_closure_zid_drop(z_closure_zid_move(&mut callback));

            // (z_info_routers_zid(self.session.loan(), z_closure_zid_move(&mut callback)) == 0)
            //     .then_some(())
            //     .ok_or(zerror!("Unable to get Routers zid"))?;

            Ok(vec![].into_iter())
        }
    }
}
