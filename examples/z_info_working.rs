use std::{ffi::CString, ptr::null_mut};

use zenoh_pico_bindings::bindings::{
    Z_CONFIG_CONNECT_KEY, Z_CONFIG_MODE_KEY, z_config_default, z_config_loan_mut, z_config_move,
    z_info_zid, z_open, z_owned_config_t, z_owned_session_t, z_session_drop, z_session_loan,
    z_session_loan_mut, z_session_move, zp_config_insert, zp_start_lease_task, zp_start_read_task,
};

fn main() {
    unsafe {
        let mut config = std::mem::MaybeUninit::<z_owned_config_t>::zeroed();
        z_config_default(config.as_mut_ptr());

        let str = CString::new("client").unwrap();
        let mode = str.as_ptr();
        zp_config_insert(
            z_config_loan_mut(config.as_mut_ptr()),
            Z_CONFIG_MODE_KEY as u8,
            mode,
        );

        let str = CString::new("tcp/127.0.0.1:7447").unwrap();
        let endpoint = str.as_ptr();
        zp_config_insert(
            z_config_loan_mut(config.as_mut_ptr()),
            Z_CONFIG_CONNECT_KEY as u8,
            endpoint,
        );

        let mut config = config.assume_init();

        let mut s = std::mem::MaybeUninit::<z_owned_session_t>::uninit();

        println!("opening session...");
        if z_open(s.as_mut_ptr(), z_config_move(&mut config), null_mut()) < 0 {
            println!("Unable to open session!");
            return;
        }

        let mut s = s.assume_init();

        println!("starting tasks...");

        if zp_start_read_task(z_session_loan_mut(&mut s), null_mut()) < 0
            || zp_start_lease_task(z_session_loan_mut(&mut s), null_mut()) < 0
        {
            println!("Unable to start read and lease tasks\n");
            z_session_drop(z_session_move(&mut s));

            return;
        }

        let id = z_info_zid(z_session_loan(&mut s));
        println!("zid: {:?}", id.id);

        println!("dropping session...");
        z_session_drop(z_session_move(&mut s));
    }
}
