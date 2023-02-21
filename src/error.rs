//! Error struct of project LCD

/* crate use */
use anyhow;
use thiserror;

/// Enum to manage error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    /// Error in logging system configuration
    #[error(transparent)]
    Log(#[from] log::SetLoggerError),

    /// Error in rayon thread pool build
    #[cfg(feature = "parallel")]
    #[error(transparent)]
    RayonThreadPool(#[from] rayon::ThreadPoolBuildError),

    /// Error in std::io
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// Error if cli parameter isn't set propely
    #[error("You must provide a count path '-p' or a kmer length '-k'")]
    NoCountNoKmer,
}

/// Alias of result
pub type Result<T> = anyhow::Result<T>;
