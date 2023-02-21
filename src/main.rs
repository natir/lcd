//! LCD: Low Coverage Detection, use kmer count to detect low coverage part of read

#![warn(missing_docs)]

/* std use */

/* crate use */

use anyhow::Context as _;
use clap::Parser as _;

/* project use */
use LCD::cli;
use LCD::error;

fn main() -> error::Result<()> {
    // parse cli
    let params = cli::Command::parse();

    // Setup logger
    stderrlog::new()
        .module(module_path!())
        .quiet(params.quiet())
        .verbosity(params.verbosity())
        .timestamp(params.timestamp())
        .init()
	.context("stderrlog already create a logger")?;

    log::error!("Hello, word!");

    Ok(())
}
