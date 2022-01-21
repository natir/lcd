//! Use kmer count to detect low coverage part of readme

/* std use */

/* crate use */

/* project use */

/* mod declaration */
// data struct
pub mod read2gap;

// subcommand
pub mod clean;
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

pub fn flatten_vec_tuple(data: &[(i32, i32)]) -> &[i32] {
    unsafe { std::mem::transmute(std::slice::from_raw_parts(data.as_ptr(), data.len() * 2)) }
}
