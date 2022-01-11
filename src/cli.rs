//! Command line interface of LCD

/* std use */

/* crate use */

/* project use */

#[derive(clap::Parser, std::fmt::Debug)]
#[clap(version = "0.1", author = "Pierre Marijon")]
/// Low Coverage Detection
pub struct Command {
    #[clap(short = 't', long = "threads")]
    /// Number of thread use by pcon to count, 0 use all avaible core, default value 0
    pub threads: Option<usize>,

    #[clap(short = 'v', long = "verbosity", parse(from_occurrences))]
    /// verbosity level also control by environment variable LCD_LOG if flag is set LCD_LOG value is ignored
    pub verbosity: i8,

    #[clap(short = 'k', long = "kmer-size")]
    /// Size of kmer
    pub kmer_size: u8,

    #[clap(short = 'i', long = "inputs")]
    /// Path to inputs
    pub inputs: Vec<std::path::PathBuf>,

    #[clap(short = 'c', long = "coverage")]
    /// If estimate coverage is equal or lower than this value are consider as low coverage region (default: 0)
    pub min_coverage: Option<u8>,

    #[clap(short = 'l', long = "gap-length")]
    /// Minimal length of coverage gap to be consider as an erroneous region (default: 20)
    pub gap_length: Option<u8>,

    #[clap(short = 'b', long = "record_buffer")]
    /// Number of sequence record load in buffer (default: 8192)
    pub buffer_length: Option<usize>,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

impl Command {
    /// Get min_coverage or default value
    pub fn min_coverage(&self) -> u8 {
        self.min_coverage.unwrap_or(0)
    }

    /// Get gap_length or default value
    pub fn gap_length(&self) -> u8 {
        self.gap_length.unwrap_or(20)
    }

    /// Get buffer_length or default value
    pub fn buffer_length(&self) -> usize {
        self.buffer_length.unwrap_or(8192)
    }
}

#[derive(clap::Parser, std::fmt::Debug)]
pub enum SubCommand {
    Detect(SubCommandDetect),
    Filter(SubCommandFilter),
    Clean(SubCommandClean),
}

#[derive(clap::Parser, std::fmt::Debug)]
/// Detect low coverage region
pub struct SubCommandDetect {}

#[derive(clap::Parser, std::fmt::Debug)]
/// Filter read with low coverage region
pub struct SubCommandFilter {}

#[derive(clap::Parser, std::fmt::Debug)]
/// Remove low coverage region
pub struct SubCommandClean {}

/// Convert verbosity count to log Level
pub fn i82level(level: i8) -> Option<log::Level> {
    match level {
        std::i8::MIN..=0 => None,
        1 => Some(log::Level::Error),
        2 => Some(log::Level::Warn),
        3 => Some(log::Level::Info),
        4 => Some(log::Level::Debug),
        5..=std::i8::MAX => Some(log::Level::Trace),
    }
}
