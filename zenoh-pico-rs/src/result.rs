use crate::bindings::z_result_t;

use zenoh_result::anyhow;
pub use zenoh_result::{Error, ZError, ZResult, to_zerror, zerror};

pub trait ToZResult {
    fn to_zerror(&self) -> ZResult<()>;
}

impl ToZResult for z_result_t {
    fn to_zerror(&self) -> ZResult<()> {
        match self {
            0 => Ok(()),
            error => {
                let error = match error {
                    -1 => "Generic error",
                    -71 => "Operation timed out",
                    -72 => "Deserialization error",
                    -73 => "Session closed",
                    -74 => "Overflow error",
                    -75 => "Invalid argument",
                    -76 => "Did not read expected amount of data",
                    -77 => "Connection closed",
                    -78 => "Out of memory",
                    -79 => "System task failed",
                    -80 => "Generic system error",
                    -87 => "No scout results",
                    -90 => "Invalid mode in configuration",
                    -91 => "Invalid locator in configuration",
                    -92 => "Unknown locator schema in configuration",
                    -93 => "Unsupported peer unicast in configuration",
                    -94 => "Unsupported client multicast in configuration",
                    -95 => "Failed to insert into configuration",
                    -97 => "Not enough bytes received from transport",
                    -98 => "No space in transport buffer",
                    -99 => "Transport receive failed",
                    -100 => "Transport transmit failed",
                    -101 => "Transport open failed due to SN resolution",
                    -102 => "Transport open failed",
                    -103 => "Transport not available",
                    -107 => "Query does not match any resource",
                    -108 => "Key expression does not match any resource",
                    -109 => "Unknown key expression",
                    -110 => "Unknown entity",
                    -111 => "Failed to declare entity",
                    -112 => "Mandatory and unknown message extension",
                    -113 => "Unknown transport in message",
                    -114 => "Unknown zenoh message type",
                    -115 => "Unknown zenoh declaration in message",
                    -116 => "Unexpected message flag",
                    -117 => "Unexpected message type",
                    -118 => "Message serialization failed",
                    -119 => "Message deserialization failed",
                    _ => "Unknown error code",
                };

                let error = zerror!(anyhow!(error));

                Err(error.into())
            }
        }
    }
}
