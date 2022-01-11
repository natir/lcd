//! Main function of LCD

/* std use */

/* crate use */
use clap::Parser as _;

/* project use */
use lcd::*;

fn main() -> error::Result<()> {
    let params = cli::Command::parse();

    if let Some(level) = cli::i82level(params.verbosity) {
        env_logger::builder()
            .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
            .filter_level(level.to_level_filter())
            .init();
    } else {
        env_logger::Builder::from_env("LCD_LOG")
            .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
            .init();
    }

    if let Some(threads) = params.threads {
        log::info!("Set number of threads to {}", threads);
        set_nb_threads(threads);
    }

    let count = if let Some(buffer_length) = params.buffer_length {
        io::count_kmer(params.kmer_size, &params.inputs, buffer_length)?
    } else {
        io::count_kmer(params.kmer_size, &params.inputs, 8192)?
    };

    match params.subcmd {
        cli::SubCommand::Detect(ref subparams) => detect::main(count, &params, subparams),
        cli::SubCommand::Filter(ref subparams) => filter::main(count, &params, subparams),
        // cli::SubCommand::Clean(subparams) => clean::clean(params),
        _ => {
            log::error!("Not supported sub command");
            Ok(())
        }
    }
}
