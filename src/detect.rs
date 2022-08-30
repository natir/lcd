//! Run detection of low coverage region

/* std use */
use std::io::Write as _;

/* crate use */
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
        target_reads,
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
    let mut histo: Vec<u64> = vec![0; 255];

    for input in inputs {
        let mut reader =
            noodles::fasta::Reader::new(std::io::BufReader::new(io::get_reader(input)?));
        let mut records_iterator = reader.records();

        let mut records: Vec<noodles::fasta::Record> = Vec::with_capacity(buffer_length);

        // Parse all read to found gap and compute coverage change
        let mut end = false;
        while !end {
            end = populate_buffer(&mut records_iterator, &mut records, buffer_length);

            log::info!("Buffer len: {}", records.len());

            let tmp_histo = records
                .par_iter()
                .filter(|r| r.sequence().len() > kmer_size as usize)
                .map(|record| {
                    let mut record_histo: Vec<u64> = vec![0; 256];

                    let kmers =
                        cocktail::tokenizer::Canonical::new(record.sequence().as_ref(), kmer_size);

                    let mut prev_count = 0;
                    let mut gap_start: Option<u8> = None;
                    for count in kmers.into_iter().map(|x| counts.get_canonic(x)) {
                        if count < min_coverage {
                            if gap_start.is_none() {
                                // Start of gap
                                gap_start = Some(prev_count);
                            }
                        } else if let Some(start) = gap_start {
                            // End of gap
                            gap_start = None;

                            record_histo[abs_diff(start, count)] += 1;
                        }

                        prev_count = count;
                    }

                    record_histo
                })
                .reduce(
                    || vec![0; 256],
                    |mut a, b| {
                        for i in 0..256 {
                            a[i] += b[i];
                        }
                        a
                    },
                );

            for i in 0..255 {
                histo[i] += tmp_histo[i];
            }

            records.clear()
        }
    }

    // Found threshold of gap
    let mut threshold = 0;
    for (i, d) in histo.windows(2).enumerate() {
        if d[1] > d[0] {
            threshold = i;
            break;
        }
    }

    let mut read2gap = read2gap::Read2Gap::default();

    for input in inputs {
        let mut reader =
            noodles::fasta::Reader::new(std::io::BufReader::new(io::get_reader(input)?));
        let mut records_iterator = reader.records();

        let mut records: Vec<noodles::fasta::Record> = Vec::with_capacity(buffer_length);

        // Parse all read to found gap and compute coverage change
        let mut end = false;
        while !end {
            end = populate_buffer(&mut records_iterator, &mut records, buffer_length);

            log::info!("Buffer len: {}", records.len());

            read2gap.0.par_extend(
                records
                    .par_iter()
                    .filter(|r| r.sequence().len() > kmer_size as usize)
                    .map(|record| {
                        let mut gap: Vec<(usize, usize)> = Vec::new();

                        let kmers = cocktail::tokenizer::Canonical::new(
                            record.sequence().as_ref(),
                            kmer_size,
                        );

                        let mut prev_count = 0;
                        let mut gap_start: Option<(usize, u8)> = None;
                        for (i, count) in
                            kmers.into_iter().map(|x| counts.get_canonic(x)).enumerate()
                        {
                            if count < min_coverage {
                                if gap_start.is_none() {
                                    // Start of gap
                                    gap_start = Some((i, prev_count));
                                }
                            } else if let Some((start_i, start_count)) = gap_start {
                                // End of gap
                                gap_start = None;

                                if abs_diff(start_count, count) > threshold
                                    && i - start_i > gap_length as usize
                                {
                                    gap.push((start_i, i));
                                }
                            }

                            prev_count = count;
                        }

                        (record.name().to_string(), gap)
                    })
                    .filter(|x| !x.1.is_empty()),
            );
        }
    }

    Ok(read2gap)
}

fn abs_diff(a: u8, b: u8) -> usize {
    if a > b {
        (a - b) as usize
    } else {
        (b - a) as usize
    }
}
