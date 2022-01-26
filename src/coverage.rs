//! Use kmer count to generate an estimate coverage of each reads

/* std use */
use std::io::Write as _;

/* crate use */
use rayon::prelude::*;

/* project use */
use crate::error::*;
use crate::*;

/// Run coverage with parameter from cli
pub fn main(
    counts: pcon::counter::Counter,
    main_params: &cli::Command,
    sub_params: &cli::SubCommandCoverage,
) -> error::Result<()> {
    log::debug!(
        "Run coverage with params: {:?} {:?}",
        main_params,
        sub_params
    );

    let target_reads = get_target_reads(&main_params.inputs, &sub_params.inputs)?;

    let mut output = std::io::BufWriter::new(sub_params.output()?);
    let coverages = coverage(
        counts,
        target_reads,
        main_params.kmer_size,
        main_params.buffer_length(),
    )?;

    for (name, pos, coverage) in coverages {
        writeln!(output, "{},{},{}", name, pos, coverage).map_err(|error| {
            Error::IO(IO::WriteError {
                path: sub_params.output_as_string(),
                error,
            })
        })?;
    }

    Ok(())
}

/// Run coverage on input files with parameter
pub fn coverage(
    counts: pcon::counter::Counter,
    inputs: &[std::path::PathBuf],
    kmer_size: u8,
    buffer_length: usize,
) -> error::Result<Vec<(String, usize, u8)>> {
    let mut reads2coverage = Vec::new();

    for input in inputs {
        let mut reader =
            noodles::fasta::Reader::new(std::io::BufReader::new(io::get_reader(input)?));
        let mut records_iterator = reader.records();

        let mut records: Vec<noodles::fasta::Record> = Vec::with_capacity(buffer_length);

        let mut end = false;
        while !end {
            end = populate_buffer(&mut records_iterator, &mut records, buffer_length);

            log::info!("Buffer len: {}", records.len());

            reads2coverage.par_extend(
                records
                    .par_iter()
                    .filter(|record| record.sequence().len() > kmer_size as usize)
                    .map(|record| {
                        cocktail::tokenizer::Canonical::new(record.sequence().as_ref(), kmer_size)
                            .enumerate()
                            .map(|(pos, canonical)| {
                                (
                                    record.name().to_string(),
                                    pos,
                                    counts.get_canonic(canonical),
                                )
                            })
                    })
                    .flatten_iter(),
            );
        }
    }

    Ok(reads2coverage)
}
