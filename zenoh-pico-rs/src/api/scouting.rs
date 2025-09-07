extern crate alloc;

use alloc::{borrow::ToOwned, vec::Vec};

use core::{ffi::CStr, ptr::null};

use zenoh_protocol::core::{Locator, WhatAmI, WhatAmIMatcher, ZenohIdProto};

use crate::{
    bindings::{
        z_closure_hello, z_closure_hello_move, z_hello_whatami, z_hello_zid, z_loaned_hello_t,
        z_owned_closure_hello_t, z_scout, z_scout_options_t, z_string_array_get,
        z_string_array_len, z_string_data, zp_hello_locators,
    },
    *,
};

pub struct ScoutOptions<I: Into<WhatAmIMatcher>> {
    pub timeout_ms: u32,
    pub what: I,
}

pub(crate) fn start_scout<I: Into<WhatAmIMatcher>>(
    mut config: Config,
    mut user_closure: impl FnMut(&ZenohIdProto, &WhatAmI, &Vec<Locator>),
    options: Option<ScoutOptions<I>>,
) -> ZResult<()> {
    let mut ffi_closure = |hello: *mut z_loaned_hello_t| {
        let (zid, whatami, locators) = unsafe {
            (
                z_hello_zid(hello),
                z_hello_whatami(hello) as u8,
                zp_hello_locators(hello),
            )
        };

        let locators = unsafe {
            let mut vec = Vec::new();

            let len = z_string_array_len(locators);

            for i in 0..len {
                let string = z_string_array_get(locators, i);
                let data = z_string_data(string);
                let cstr = CStr::from_ptr(data).to_str().ok().map(|s| s.to_owned());

                if let Some(string) = cstr {
                    if let Ok(locator) = Locator::try_from(string) {
                        vec.push(locator);
                    }
                }
            }

            vec
        };

        if let Ok(zid) = ZenohIdProto::try_from(zid.id) {
            if let Ok(whatami) = WhatAmI::try_from(whatami) {
                user_closure(&zid, &whatami, &locators);
            }
        }
    };

    let (data, ffi_closure_ptr) = ffi_closure.to_ffi();

    let mut closure = core::mem::MaybeUninit::<z_owned_closure_hello_t>::uninit();

    let mut closure = unsafe {
        z_closure_hello(closure.as_mut_ptr(), Some(ffi_closure_ptr), None, data).to_zerror()?;

        closure.assume_init()
    };

    let options = options.map(|opts| z_scout_options_t {
        timeout_ms: opts.timeout_ms,
        what: (u8::from(opts.what.into())) as u32,
    });

    unsafe {
        z_scout(
            config.take(),
            z_closure_hello_move(&mut closure),
            match options {
                Some(ref o) => o as *const z_scout_options_t,
                None => null(),
            },
        )
        .to_zerror()?
    };

    Ok(())
}
