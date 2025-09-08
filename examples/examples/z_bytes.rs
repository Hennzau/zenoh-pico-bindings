use zenoh_pico_rs::{ZBytes, ZResult};

fn main() -> ZResult<()> {
    let _bytes = {
        let mut vec = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        vec.reserve(200);

        println!("vec: {:?}", vec);
        ZBytes::try_from(vec)?
    };

    Ok(())
}
