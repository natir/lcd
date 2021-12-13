//! Run cleaning of low coverage region inside of read

/* std use */

/* crate use */

/* project use */
use crate::*;

pub fn clean(params: cli::SubCommandClean) -> error::Result<()> {
    log::debug!("Run clean with params: {:?}", params);

    Ok(())
}
