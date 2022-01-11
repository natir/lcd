//! LCD error enum's

/* std use */

/* crate use */

/* project use */

#[derive(thiserror::Error, std::fmt::Debug)]
pub enum Error {
    /// Input Output error [IO]
    #[error(transparent)]
    IO(#[from] IO),

    /// Error associate to Command Line Interface [Cli]
    #[error(transparent)]
    Cli(#[from] Cli),

    #[error(transparent)]
    Transparent(#[from] Box<dyn std::error::Error>),
}

#[derive(thiserror::Error, std::fmt::Debug)]
pub enum Cli {
    /// Can't convert str in OutputFormat
    #[error("Can't convert \"{params}\" in OutputFormat allow value are 'text', 'json'")]
    OutputFormatCast { params: String },

    /// Number of input and output must be equal
    #[error("Number of input ({input}) and output ({output}) must be equal")]
    DiffInputOutput { input: usize, output: usize },
}

#[derive(thiserror::Error, std::fmt::Debug)]
pub enum IO {
    /// Can't open file.
    #[error("Can't open file {path}, error: {error}")]
    CantOpenFile { path: String, error: std::io::Error },

    /// Write error
    #[error("Error durring write in file {path}, error: {error}")]
    WriteError { path: String, error: std::io::Error },
}

pub type Result<T> = std::result::Result<T, Error>;
