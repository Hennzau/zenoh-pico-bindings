use std::{ffi::CString, ptr::null_mut};

use zenoh_pico_bindings::{
    ToZResult, Z_CONFIG_CONNECT_KEY, Z_CONFIG_MODE_KEY, ZResult, z_config_default,
    z_config_loan_mut, z_config_move, z_id_t, z_info_zid, z_open, z_owned_config_t,
    z_owned_session_t, z_session_drop, z_session_loan, z_session_loan_mut, z_session_move,
    zp_config_insert, zp_start_lease_task, zp_start_read_task,
};

fn create_config() -> ZResult<z_owned_config_t> {
    unsafe {
        let mut config = std::mem::MaybeUninit::<z_owned_config_t>::uninit();
        z_config_default(config.as_mut_ptr()).to_zerror()?;

        let str = CString::new("client").unwrap();
        let mode = str.as_ptr();
        zp_config_insert(
            z_config_loan_mut(config.as_mut_ptr()),
            Z_CONFIG_MODE_KEY as u8,
            mode,
        )
        .to_zerror()?;

        let str = CString::new("tcp/127.0.0.1:7447").unwrap();
        let endpoint = str.as_ptr();
        zp_config_insert(
            z_config_loan_mut(config.as_mut_ptr()),
            Z_CONFIG_CONNECT_KEY as u8,
            endpoint,
        )
        .to_zerror()?;

        Ok(config.assume_init())
    }
}

fn create_session(mut config: z_owned_config_t) -> ZResult<z_owned_session_t> {
    unsafe {
        let mut s = std::mem::MaybeUninit::<z_owned_session_t>::uninit();

        println!("opening session...");
        z_open(s.as_mut_ptr(), z_config_move(&mut config), null_mut()).to_zerror()?;

        Ok(s.assume_init())
    }
}

fn start_tasks(session: &mut z_owned_session_t) -> ZResult<()> {
    unsafe {
        println!("starting tasks...");

        zp_start_read_task(z_session_loan_mut(session), null_mut()).to_zerror()?;
        zp_start_lease_task(z_session_loan_mut(session), null_mut()).to_zerror()?;
    }

    Ok(())
}

fn zid(session: &z_owned_session_t) -> z_id_t {
    unsafe { z_info_zid(z_session_loan(session)) }
}

fn drop_session(mut session: z_owned_session_t) {
    unsafe {
        println!("dropping session...");

        z_session_drop(z_session_move(&mut session));
    }
}

fn main() -> ZResult<()> {
    let config = create_config()?;
    let mut session = create_session(config)?;

    start_tasks(&mut session).or_else(|e| {
        drop_session(session);

        Err(e)
    })?;

    let id = zid(&session);
    println!("zid: {:?}", id.id);

    drop_session(session);

    Ok(())
}
