//! Run filtration of read with low coverage region or low coverage

/* std use */

/* crate use */

/* project use */
use crate::*;

/// Run filter with parameter from cli
pub fn main(
    counts: pcon::counter::Counter,
    main_params: &cli::Command,
    sub_params: &cli::SubCommandFilter,
) -> error::Result<()> {
    log::debug!("Run filter with params: {:?} {:?}", main_params, sub_params);

    let read2gap = detect::detect(
        counts,
        &main_params.inputs,
        main_params.kmer_size,
        main_params.min_coverage(),
        main_params.gap_length(),
        main_params.buffer_length(),
    )?;

    filter(&read2gap, &main_params.inputs, sub_params.outputs()?)?;

    Ok(())
}

/// Filter sequence in inputs based on read2gap information write valid read in outputs
pub fn filter(
    read2gap: &read2gap::Read2Gap,
    inputs: &[std::path::PathBuf],
    outputs: Vec<Box<dyn std::io::Write>>,
) -> error::Result<()> {
    Ok(())
}
