//! Run filtration of read with low coverage region or low coverage

/* std use */

/* crate use */
use rayon::prelude::*;

/* project use */
use crate::error::*;
use crate::*;

/// Run filter with parameter from cli
pub fn main(
    counts: pcon::counter::Counter,
    main_params: &cli::Command,
    sub_params: &cli::SubCommandFilter,
) -> error::Result<()> {
    log::debug!("Run filter with params: {:?} {:?}", main_params, sub_params);

    if main_params.inputs.len() != sub_params.outputs()?.len() {
        return Err(Error::Cli(Cli::DiffInputOutput {
            input: main_params.inputs.len(),
            output: sub_params.outputs()?.len(),
        }));
    }

    let read2gap = detect::detect(
        counts,
        &main_params.inputs,
        main_params.kmer_size,
        main_params.min_coverage(),
        main_params.gap_length(),
        main_params.buffer_length(),
    )?;

    filter(
        &read2gap,
        &main_params.inputs,
        sub_params.outputs()?,
        main_params.buffer_length(),
    )?;

    Ok(())
}

/// Filter sequence in inputs based on read2gap information write valid read in outputs
pub fn filter(
    read2gap: &read2gap::Read2Gap,
    inputs: &[std::path::PathBuf],
    mut outputs: Vec<Box<dyn std::io::Write>>,
    buffer_length: usize,
) -> error::Result<()> {
    for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
        let mut reader =
            noodles::fasta::Reader::new(std::io::BufReader::new(io::get_reader(input)?));
        let mut writer = noodles::fasta::Writer::new(std::io::BufWriter::new(output));

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

            let keep_record: Vec<&noodles::fasta::Record> = records
                .par_iter()
                .filter(|record| !read2gap.0.contains_key(record.name()))
                .collect();

            for record in keep_record.iter() {
                writer.write_record(record).map_err(|error| {
                    Error::IO(IO::WriteError {
                        path: "Unknow sorry".to_string(),
                        error,
                    })
                })?;
            }
        }
    }

    Ok(())
}
