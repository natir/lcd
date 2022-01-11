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
