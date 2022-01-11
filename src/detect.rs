//! Run detection of low coverage region

/* std use */

/* crate use */
use rayon::prelude::*;

/* project use */
use crate::*;

/// Run detection with parameter from cli
pub fn main(
    counts: pcon::counter::Counter,
    main_params: &cli::Command,
    sub_params: &cli::SubCommandDetect,
) -> error::Result<()> {
    log::debug!("Run detect with params: {:?} {:?}", main_params, sub_params);

    let read2gap = detect(
        counts,
        &main_params.inputs,
        main_params.kmer_size,
        main_params.min_coverage(),
        main_params.gap_length(),
        main_params.buffer_length(),
    )?;

    for (name, gap) in read2gap.0 {
        println!("{}: {:?}", name, gap);
    }

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

    for input in inputs.iter() {
        let mut reader =
            noodles::fasta::Reader::new(std::io::BufReader::new(io::get_reader(input)?));
        let mut records_iterator = reader.records();

        let mut records: Vec<noodles::fasta::Record> = Vec::with_capacity(buffer_length);

        let mut end = false;
        while !end {
            for i in 0..buffer_length {
                if let Some(Ok(record)) = records_iterator.next() {
                    records.push(record);
                } else {
                    end = true;
                    records.truncate(i);
                    break;
                }
            }

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

                            let mut begin_gap: Option<usize> = None;
                            for (i, canonical) in tokenizer.enumerate() {
                                let coverage = counts.get_canonic(canonical);

                                if begin_gap.is_none() && coverage < min_coverage {
                                    begin_gap = Some(i)
                                }

                                if coverage > min_coverage {
                                    if let Some(begin) = begin_gap {
                                        if i - begin > gap_length as usize {
                                            gap.push((begin, i));
                                        }
                                        begin_gap = None
                                    }
                                }
                            }
                        }

                        (record.name().to_string(), gap)
                    })
                    .filter(|x| !x.1.is_empty()),
            );
        }
    }

    Ok(read2gap)
}
