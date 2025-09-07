use clap::Parser;
use zenoh_pico_rs_examples::CommonArgs;

use zenoh_pico_rs::{protocol::OwnedKeyExpr, *};

fn main() -> ZResult<()> {
    let (config, key_expr, payload) = parse_args()?;

    println!("Opening session...");
    let session = open(config)?;

    println!("Putting Data ('{key_expr}': '{payload}')...");
    session.put(&key_expr, payload.into_bytes())?;

    Ok(())
}

#[derive(clap::Parser, Clone, PartialEq, Eq, Hash, Debug)]
struct Args {
    #[arg(short, long, default_value = "demo/example/zenoh-pico-put")]
    key: OwnedKeyExpr,
    #[arg(short, long, default_value = "Put from Rust!")]
    payload: String,
    #[command(flatten)]
    common: CommonArgs,
}

fn parse_args() -> ZResult<(Config, OwnedKeyExpr, String)> {
    let args = Args::parse();

    Ok((args.common.try_into()?, args.key, args.payload))
}
