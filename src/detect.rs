//! Run detection of low coverage region

/* std use */
use std::io::Write as _;

/* crate use */
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray_stats::EntropyExt;
use rayon::prelude::*;

/* project use */
use crate::error::*;
use crate::*;

/// Run detection with parameter from cli
pub fn main(
    counts: pcon::counter::Counter,
    main_params: &cli::Command,
    sub_params: &cli::SubCommandDetect,
) -> error::Result<()> {
    log::debug!("Run detect with params: {:?} {:?}", main_params, sub_params);

    let target_reads = get_target_reads(&main_params.inputs, &sub_params.inputs)?;

    let read2gap = detect(
        counts,
        &target_reads,
        main_params.kmer_size,
        main_params.min_coverage(),
        main_params.gap_length(),
        main_params.buffer_length(),
    )?;

    writeln!(
        std::io::BufWriter::new(sub_params.output()?),
        "{}",
        match sub_params.format {
            Some(cli::OutputFormat::Text) | None => read2gap.text()?,
            Some(cli::OutputFormat::Json) => read2gap.json()?,
        }
    )
    .map_err(|error| {
        Error::IO(IO::WriteError {
            path: sub_params.output_as_string(),
            error,
        })
    })?;

    Ok(())
}

/// Run detection on input files with parameter
pub fn detect(
    counts: pcon::counter::Counter,
    inputs: &[std::path::PathBuf],
    kmer_size: u8,
    min_coverage: u8,
    gap_length: u8,
    buffer_length: usize,
) -> error::Result<read2gap::Read2Gap> {
    let mut read2gap = read2gap::Read2Gap::default();

    for input in inputs {
        let mut reader =
            noodles::fasta::Reader::new(std::io::BufReader::new(io::get_reader(input)?));
        let mut records_iterator = reader.records();

        let mut records: Vec<noodles::fasta::Record> = Vec::with_capacity(buffer_length);

        let mut end = false;
        while !end {
            end = populate_buffer(&mut records_iterator, &mut records, buffer_length);

            log::info!("Buffer len: {}", records.len());

            read2gap.0.par_extend(
                records
                    .par_iter()
                    .map(|record| {
                        let mut gap: Vec<(usize, usize)> = Vec::new();

                        if record.sequence().len() >= kmer_size as usize {
                            let tokenizer = cocktail::tokenizer::Canonical::new(
                                record.sequence().as_ref(),
                                kmer_size,
                            );

                            let cover: ndarray::Array<f64, ndarray::Ix1> = tokenizer
                                .map(|cano| counts.get_canonic(cano) as f64)
                                .collect();
                            let total_cover = cover.sum();
                            let reality = cover / total_cover;
                            let theory = ndarray::Array::from_iter(
                                std::iter::repeat(
                                    total_cover / record.sequence().len() as f64 / total_cover,
                                )
                                .take(record.sequence().len() - kmer_size as usize + 1),
                            );

                            let divergence = reality.kl_divergence(&theory).unwrap(); //_or(1.0);

                            println!("{}\n{}\n{}\n{}", record.name(), theory, reality, divergence);

                            if divergence > 0.5 {
                                gap.push((0, record.sequence().len()));
                            }
                        }

                        (record.name().to_string(), gap)
                    })
                    .filter(|x| !x.1.is_empty()),
            );

            records.clear()
        }
    }

    Ok(read2gap)
}
