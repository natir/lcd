//! LCD error enum's

/* std use */

/* crate use */

/* project use */

#[derive(thiserror::Error, std::fmt::Debug)]
pub enum Error {
    /// Input Output error [IO]
    #[error(transparent)]
    IO(#[from] IO),

    #[error(transparent)]
    Transparent(#[from] Box<dyn std::error::Error>),
}

#[derive(thiserror::Error, std::fmt::Debug)]
pub enum IO {
    /// I can't open file.
    #[error("I can't open file {path:?}, error: {error}")]
    CantOpenFile {
        path: std::path::PathBuf,
        error: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
