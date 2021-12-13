//! Use kmer count to detect low coverage part of readme

/* std use */

/* crate use */

/* project use */

/* mod declaration */
pub mod clean;
pub mod cli;
pub mod detect;
pub mod error;
pub mod filter;
pub mod io;

/// Set the number of threads use by pcon
pub fn set_nb_threads(nb_threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(nb_threads)
        .build_global()
        .unwrap();
}
