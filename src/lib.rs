//! Use kmer count to detect low coverage part of readme

/* std use */

/* crate use */

/* project use */

/* mod declaration */
// data struct
pub mod read2gap;

// subcommand
pub mod clean;
pub mod coverage;
pub mod detect;
pub mod filter;

// utils
pub mod cli;
pub mod error;
pub mod io;

/// Set the number of threads use by pcon
pub fn set_nb_threads(nb_threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(nb_threads)
        .build_global()
        .unwrap();
}

/// Fill record buffer, with record from input
pub fn populate_buffer<T>(
    input: &mut noodles::fasta::reader::Records<T>,
    records: &mut Vec<noodles::fasta::Record>,
    buffer_length: usize,
) -> bool
where
    T: std::io::BufRead,
{
    for i in 0..buffer_length {
        if let Some(Ok(record)) = input.next() {
            records.push(record);
        } else {
            records.truncate(i);
            return true;
        }
    }

    false
}

/// Get good set of input reads
pub fn get_target_reads<'a>(
    main_inputs: &'a Option<Vec<std::path::PathBuf>>,
    sub_inputs: &'a Option<Vec<std::path::PathBuf>>,
) -> error::Result<&'a Vec<std::path::PathBuf>> {
    match (main_inputs.as_ref(), sub_inputs.as_ref()) {
        (Some(_), Some(sub)) => Ok(sub),
        (None, Some(sub)) => Ok(sub),
        (Some(main), None) => Ok(main),
        _ => Err(error::Error::Cli(
            error::Cli::InputsIsRequiredInMainOrSubCommand,
        )),
    }
}
