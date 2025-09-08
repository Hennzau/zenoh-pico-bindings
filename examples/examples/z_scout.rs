use zenoh_pico_rs::*;

fn main() -> ZResult<()> {
    let config = Config::default()?;

    scout(
        config,
        |zid: &ZenohIdProto, what_am_i: &WhatAmI, locators: &Vec<Locator>| {
            println!("Discovered entity:");

            println!("  zid: {}", zid);
            println!("  what_am_i: {:?}", what_am_i);
            println!("  locators: {:?}", locators);
        },
        Some(ScoutOptions {
            timeout_ms: 100,
            what: WhatAmI::Client | WhatAmI::Router | WhatAmI::Peer,
        }),
    )?;

    Ok(())
}
