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

    let count = match (&params.inputs, &params.counts) {
        (Some(paths), _) => io::count_kmer(params.kmer_size, paths, params.buffer_length())?,
        (_, Some(path)) => {
            let mut reader = std::io::BufReader::new(io::get_reader(path)?);
            pcon::counter::Counter::deserialize(&mut reader)
                .map_err(|error| error::Error::Transparent(error.into()))?
        }
        _ => return Err(error::Error::Cli(error::Cli::InputsOrCountMustBeSet)),
    };

    match params.subcmd {
        cli::SubCommand::Coverage(ref subparams) => coverage::main(count, &params, subparams),
        cli::SubCommand::Detect(ref subparams) => detect::main(count, &params, subparams),
        cli::SubCommand::Filter(ref subparams) => filter::main(count, &params, subparams),
        cli::SubCommand::Clean(ref subparams) => clean::main(count, &params, subparams),
    }
}
