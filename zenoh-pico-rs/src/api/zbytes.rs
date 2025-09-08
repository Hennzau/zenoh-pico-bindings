use crate::{
    bindings::{z_bytes_drop, z_bytes_from_buf, z_bytes_move, z_moved_bytes_t, z_owned_bytes_t},
    *,
};

use alloc::{boxed::Box, vec::Vec};
use core::ffi::c_void;

pub struct ZBytes {
    zp: z_owned_bytes_t,
    moved: bool,
}

impl ZBytes {
    pub(crate) fn new(bytes: z_owned_bytes_t) -> Self {
        Self {
            zp: bytes,
            moved: false,
        }
    }

    pub(crate) fn take(&mut self) -> *mut z_moved_bytes_t {
        if self.moved {
            panic!("ZBytes has already been moved");
        }

        self.moved = true;
        unsafe { z_bytes_move(&mut self.zp) }
    }
}

impl TryFrom<Box<[u8]>> for ZBytes {
    type Error = Error;

    fn try_from(value: Box<[u8]>) -> ZResult<Self> {
        let len = value.len();
        let data = Box::into_raw(value);

        unsafe extern "C" fn deleter(data: *mut c_void, context: *mut c_void) {
            let len = context as usize;
            let slice = unsafe { core::slice::from_raw_parts_mut(data as *mut u8, len) };

            unsafe { drop(Box::from_raw(slice as *mut [u8])) };
        }

        let mut bytes = core::mem::MaybeUninit::<z_owned_bytes_t>::uninit();
        let bytes = unsafe {
            z_bytes_from_buf(
                bytes.as_mut_ptr(),
                data as *mut _,
                len,
                Some(deleter),
                len as *mut _,
            )
            .to_zerror()?;
            bytes.assume_init()
        };

        Ok(Self::new(bytes))
    }
}

impl TryFrom<Vec<u8>> for ZBytes {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> ZResult<Self> {
        match value.capacity() == value.len() {
            true => Self::try_from(value.into_boxed_slice()),
            false => {
                let len = value.len();
                let capacity = value.capacity();

                if capacity < (1 << (usize::BITS / 2)) {
                    let value = core::mem::ManuallyDrop::new(value);
                    let combined: usize = (capacity << 32) | len;

                    unsafe extern "C" fn deleter(data: *mut c_void, context: *mut c_void) {
                        let combined = context as usize;
                        let len = combined & ((1 << (usize::BITS / 2)) - 1);
                        let capacity = combined >> (usize::BITS / 2);

                        let vec = unsafe { Vec::from_raw_parts(data as *mut u8, len, capacity) };
                        drop(vec);
                    }

                    let mut bytes = core::mem::MaybeUninit::<z_owned_bytes_t>::uninit();
                    let bytes = unsafe {
                        z_bytes_from_buf(
                            bytes.as_mut_ptr(),
                            value.as_ptr() as *mut _,
                            len,
                            Some(deleter),
                            combined as *mut _,
                        )
                        .to_zerror()?;
                        bytes.assume_init()
                    };

                    Ok(Self::new(bytes))
                } else {
                    let boxed = Box::new(value);
                    let data = boxed.as_ptr();

                    let boxed_ptr = Box::into_raw(boxed);

                    unsafe extern "C" fn deleter(_data: *mut c_void, context: *mut c_void) {
                        let boxed_ptr = context as *mut Vec<u8>;
                        unsafe { drop(Box::from_raw(boxed_ptr)) };
                    }

                    let mut bytes = core::mem::MaybeUninit::<z_owned_bytes_t>::uninit();
                    let bytes = unsafe {
                        z_bytes_from_buf(
                            bytes.as_mut_ptr(),
                            data as *mut _,
                            len,
                            Some(deleter),
                            boxed_ptr as *mut _,
                        )
                        .to_zerror()?;
                        bytes.assume_init()
                    };

                    Ok(Self::new(bytes))
                }
            }
        }
    }
}

impl TryFrom<&[u8]> for ZBytes {
    type Error = Error;

    fn try_from(value: &[u8]) -> ZResult<Self> {
        Self::try_from(value.to_vec())
    }
}

impl TryFrom<alloc::string::String> for ZBytes {
    type Error = Error;

    fn try_from(value: alloc::string::String) -> ZResult<Self> {
        Self::try_from(value.into_bytes())
    }
}

impl TryFrom<&str> for ZBytes {
    type Error = Error;

    fn try_from(value: &str) -> ZResult<Self> {
        Self::try_from(value.as_bytes())
    }
}

impl Drop for ZBytes {
    fn drop(&mut self) {
        if !self.moved {
            unsafe {
                z_bytes_drop(self.take());
            }
        }
    }
}
