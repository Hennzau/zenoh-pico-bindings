use clap::Parser;
use zenoh_pico_rs_examples::CommonArgs;

use zenoh_pico_rs::*;

fn main() -> ZResult<()> {
    let config = parse_args()?;

    println!("Opening session...");
    let session = open(config)?;

    let info = session.info();
    println!("zid: {}", info.zid()?);
    println!("routers zid: {:?}", info.routers_zid()?);
    println!("peers zid: {:?}", info.peers_zid()?);

    Ok(())
}

#[derive(clap::Parser, Clone, PartialEq, Eq, Hash, Debug)]
struct Args {
    #[command(flatten)]
    common: CommonArgs,
}

fn parse_args() -> ZResult<Config> {
    let args = Args::parse();

    args.common.try_into()
}
