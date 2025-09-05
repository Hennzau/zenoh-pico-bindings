use zenoh_pico_rs::{protocol::*, *};

#[derive(clap::Parser, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CommonArgs {
    #[arg(short, long)]
    mode: Option<WhatAmI>,

    #[arg(short = 'e', long)]
    connect: Vec<String>,

    #[arg(short, long)]
    listen: Option<String>,
}

impl TryFrom<CommonArgs> for Config {
    type Error = Error;
    fn try_from(value: CommonArgs) -> ZResult<Self> {
        (&value).try_into()
    }
}

impl TryFrom<&CommonArgs> for Config {
    type Error = Error;
    fn try_from(args: &CommonArgs) -> ZResult<Self> {
        let mut config = Config::default()?;

        if let Some(mode) = args.mode {
            config.insert(
                ConfigKey::Mode,
                match mode {
                    WhatAmI::Client => ValueKey::Client,
                    WhatAmI::Peer => ValueKey::Peer,
                    WhatAmI::Router => {
                        return Err(zerror!("Router mode is not supported in zenoh-pico.").into());
                    }
                },
            )?;
        }

        for endpoint in &args.connect {
            config.insert(
                ConfigKey::Connect,
                ValueKey::Endpoint(endpoint.as_str().into()),
            )?;
        }

        if let Some(listen) = &args.listen {
            config.insert(
                ConfigKey::Listen,
                ValueKey::Endpoint(listen.as_str().into()),
            )?;
        }

        Ok(config)
    }
}
