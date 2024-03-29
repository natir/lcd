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

    let target_reads = get_target_reads(&main_params.inputs, &sub_params.inputs)?;

    if target_reads.len() != sub_params.outputs()?.len() {
        return Err(Error::Cli(Cli::DiffInputOutput {
            input: target_reads.len(),
            output: sub_params.outputs()?.len(),
        }));
    }

    let read2gap = detect::detect(
        counts,
        target_reads,
        main_params.kmer_size,
        main_params.min_coverage(),
        main_params.gap_length(),
        main_params.buffer_length(),
    )?;

    filter(
        &read2gap,
        target_reads,
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
            end = populate_buffer(&mut records_iterator, &mut records, buffer_length);

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

            records.clear();
        }
    }

    Ok(())
}
