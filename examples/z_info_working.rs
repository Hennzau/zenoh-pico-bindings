use std::{error::Error, ffi::CString, ptr::null_mut};

use zenoh_pico_bindings::{
    ToZResult, Z_CONFIG_CONNECT_KEY, ZResult, z_config_default, z_config_drop, z_config_loan_mut,
    z_config_move, z_info_zid, z_open, z_owned_config_t, z_owned_session_t, z_session_drop,
    z_session_loan, z_session_loan_mut, z_session_move, zp_config_insert, zp_start_lease_task,
    zp_start_read_task,
};

fn config_drop(e: Box<dyn Error + Send + Sync>, config: &mut z_owned_config_t) -> ZResult<()> {
    unsafe { z_config_drop(z_config_move(config)) };

    Err(e)
}

fn session_drop(e: Box<dyn Error + Send + Sync>, session: &mut z_owned_session_t) -> ZResult<()> {
    unsafe { z_session_drop(z_session_move(session)) };

    Err(e)
}

fn main() -> ZResult<()> {
    unsafe {
        let mut config = std::mem::MaybeUninit::<z_owned_config_t>::zeroed();
        z_config_default(config.as_mut_ptr()).to_zerror()?;
        let mut config = config.assume_init();

        let str = CString::new("tcp/127.0.0.1:7447").unwrap();
        let endpoint = str.as_ptr();
        zp_config_insert(
            z_config_loan_mut(&mut config),
            Z_CONFIG_CONNECT_KEY as u8,
            endpoint,
        )
        .to_zerror()
        .or_else(|e| config_drop(e, &mut config))?;

        println!("opening session...");

        let mut s = std::mem::MaybeUninit::<z_owned_session_t>::uninit();
        z_open(s.as_mut_ptr(), z_config_move(&mut config), null_mut()).to_zerror()?;
        let mut s = s.assume_init();

        println!("starting tasks...");
        zp_start_read_task(z_session_loan_mut(&mut s), null_mut())
            .to_zerror()
            .or_else(|e| session_drop(e, &mut s))?;
        zp_start_lease_task(z_session_loan_mut(&mut s), null_mut())
            .to_zerror()
            .or_else(|e| session_drop(e, &mut s))?;

        let id = z_info_zid(z_session_loan(&mut s));
        println!("zid: {:?}", id.id);

        println!("dropping session...");
        z_session_drop(z_session_move(&mut s));
    }

    Ok(())
}
