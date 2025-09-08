use core::ptr::null_mut;

use zenoh_buffers::{ZBuf, buffer::Buffer};

use crate::{
    bindings::{z_bytes_from_buf, z_bytes_move, z_moved_bytes_t, z_owned_bytes_t},
    *,
};

pub(crate) struct Payload<'a> {
    _zbuf: &'a ZBuf,

    zp: z_owned_bytes_t,
    moved: bool,
}

impl Payload<'_> {
    pub(crate) fn take(&mut self) -> *mut z_moved_bytes_t {
        if self.moved {
            panic!("Payload already moved");
        }

        self.moved = true;
        unsafe { z_bytes_move(&mut self.zp) }
    }
}

pub(crate) fn make_payload<'a>(zbuf: &'a ZBuf) -> ZResult<Payload<'a>> {
    let mut payload = core::mem::MaybeUninit::<z_owned_bytes_t>::uninit();

    let payload = unsafe {
        z_bytes_from_buf(
            payload.as_mut_ptr(),
            zbuf.to_zslice().as_ptr() as *mut u8,
            zbuf.len(),
            None,
            null_mut(),
        )
        .to_zerror()?;

        payload.assume_init()
    };

    Ok(Payload {
        _zbuf: zbuf,
        zp: payload,
        moved: false,
    })
}
