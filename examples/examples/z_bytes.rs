use zenoh_pico_rs::{
    ZBytes, ZResult,
    bindings::{
        z_bytes_to_slice, z_owned_slice_t, z_slice_data, z_slice_drop, z_slice_len, z_slice_loan,
        z_slice_move,
    },
};

fn main() -> ZResult<()> {
    unsafe {
        let bytes = {
            let mut vec = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            vec.reserve(200);

            println!("vec: {:?}", vec);
            ZBytes::try_from(vec)?
        };

        let mut z_slice = {
            let mut z_slice = core::mem::MaybeUninit::<z_owned_slice_t>::uninit();
            z_bytes_to_slice(bytes.loan(), z_slice.as_mut_ptr());
            let z_slice = z_slice.assume_init();

            z_slice
        };

        let data = z_slice_data(z_slice_loan(&z_slice));
        let len = z_slice_len(z_slice_loan(&z_slice));

        let slice = core::slice::from_raw_parts(data, len as usize);
        println!("z_bytes to z_slice: {:?}", slice);

        z_slice_drop(z_slice_move(&mut z_slice));

        let mut test_vec = vec![
            72u8, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33,
        ];
        test_vec.reserve(200);
        let boxed = test_vec.into_boxed_slice();
        println!("boxed: {:?}", boxed);

        // let bytes = {
        //     let string = String::from("Hello, world!");

        //     ZBytes::try_from(string)?
        // };

        // let mut z_str = {
        //     let mut z_str = core::mem::MaybeUninit::<z_owned_string_t>::uninit();
        //     z_bytes_to_string(bytes.loan(), z_str.as_mut_ptr());
        //     let z_str = z_str.assume_init();

        //     z_str
        // };

        // let ffi_string = z_string_data(z_string_loan(&z_str)) as *mut i8;
        // let ffi_string = std::ffi::CStr::from_ptr(ffi_string);

        // println!("z_bytes to z_string: {:?}", ffi_string);
        // z_string_drop(z_string_move(&mut z_str));
    }

    Ok(())
}
