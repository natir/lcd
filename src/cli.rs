//! Command Line Interface declaration of project LCD

/* std use */
use std::io::Read as _;

/* crate use */

/* project use */
use crate::error;

/* mod declaration */
pub mod coverage;

/* pub use */
pub use coverage::Coverage;

/// LCD: Low Coverage Detection, use kmer count to detect low coverage part of read
#[derive(clap::Parser, std::fmt::Debug)]
#[clap(
    name = "lcd",
    version = "0.1",
    author = "Pierre Marijon <pierre@marijon.fr>"
)]
pub struct Command {
    /* Generic argument */
    #[cfg(feature = "parallel")]
    /// Number of thread use 0 use all avaible core, default value 0
    #[clap(short = 't', long = "threads")]
    threads: Option<usize>,

    /// Silence all output
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[clap(short = 'v', long = "verbosity", action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Timestamp (sec, ms, ns, none)
    #[clap(short = 'T', long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,

    /* specific argument */
    /// Path to count produce by pcon
    #[clap(short = 'p', long = "pcon")]
    pub count: Option<std::path::PathBuf>,

    /// Estimate coverage, equal or lower are consider as low coverage (default: 0)
    #[clap(short = 'm', long = "min-coverage")]
    pub min_coverage: Option<pcon::CountTypeNoAtomic>,

    /// Minimal length of coverage gap (default: 20)
    #[clap(short = 'l', long = "gap-length")]
    pub gap_length: Option<u64>,

    /// Path to fasta file to be analyse
    #[clap(short = 'i', long = "inputs")]
    pub inputs: Option<Vec<std::path::PathBuf>>,

    /// Kmer length if you didn't provide solidity path you must give a kmer length
    #[clap(short = 'k', long = "kmer-size")]
    pub kmer_size: Option<u8>,

    /// Number of sequence record load in buffer, default 8192
    #[clap(short = 'b', long = "record_buffer")]
    pub record_buffer: Option<u64>,

    /// SubCommand
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

impl Command {
    #[cfg(feature = "parallel")]
    /// Get number of thread
    pub fn threads(&self) -> usize {
        self.threads.unwrap_or(0)
    }

    /// Get verbosity level
    pub fn verbosity(&self) -> usize {
        self.verbosity as usize
    }

    /// Get quiet
    pub fn quiet(&self) -> bool {
        self.quiet
    }

    /// Get timestamp granularity
    pub fn timestamp(&self) -> stderrlog::Timestamp {
        self.ts.unwrap_or(stderrlog::Timestamp::Off)
    }

    /// Read or compute count
    pub fn count(&self) -> error::Result<pcon::counter::Counter<pcon::CountType>> {
        if let Some(count_path) = &self.count {
            log::info!("Load count file");
            pcon::counter::Counter::<pcon::CountType>::from_stream(open(count_path)?)
        } else if let Some(kmer_size) = self.kmer_size {
            let mut counter = pcon::counter::Counter::<pcon::CountType>::new(kmer_size);

            log::info!("Start count kmer from input");
            counter.count_fasta(self.input()?, self.record_buffer());
            log::info!("End count kmer from input");

            Ok(counter)
        } else {
            Err(error::Error::NoCountNoKmer.into())
        }
    }

    /// Get inputs
    pub fn input(&self) -> error::Result<Box<dyn std::io::BufRead>> {
        match &self.inputs {
            None => Ok(Box::new(std::io::stdin().lock())),
            Some(paths) => {
                let mut handle: Box<dyn std::io::Read> = Box::new(std::io::Cursor::new(vec![]));

                for path in paths {
                    let file = open(path)?;
                    handle = Box::new(handle.chain(file));
                }

                Ok(Box::new(std::io::BufReader::new(handle)))
            }
        }
    }

    /// Get record_buffer
    pub fn record_buffer(&self) -> u64 {
        self.record_buffer.unwrap_or(8192)
    }
}

/// Enumeration of subcommand
#[derive(clap::Subcommand, std::fmt::Debug)]
pub enum SubCommand {
    /// Detect coverage gap
    Coverage(Coverage),
}

fn create<P>(path: P) -> error::Result<Box<dyn std::io::Write + std::marker::Send>>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let file = std::fs::File::create(path)?;
    let buffer = std::io::BufWriter::new(file);
    let boxed = Box::new(buffer);

    Ok(boxed)
}

fn open<P>(path: P) -> error::Result<Box<dyn std::io::Read + std::marker::Send>>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let file = std::fs::File::open(path)?;
    let boxed = Box::new(file);
    let (niffle, _) = niffler::send::get_reader(boxed)?;
    let buffer = std::io::BufReader::new(niffle);

    Ok(Box::new(buffer))
}
