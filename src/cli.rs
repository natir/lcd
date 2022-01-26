//! Command line interface of LCD

/* std use */

/* crate use */

/* project use */
use crate::*;

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
    /// Paths to the sequence files where the kmer will be counted
    pub inputs: Option<Vec<std::path::PathBuf>>,

    #[clap(short = 'c', long = "counts")]
    /// A previous pcon count
    pub counts: Option<std::path::PathBuf>,

    #[clap(short = 't', long = "threshold")]
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
    Coverage(SubCommandCoverage),
    Detect(SubCommandDetect),
    Filter(SubCommandFilter),
    Clean(SubCommandClean),
}

#[derive(clap::Parser, std::fmt::Debug)]
/// Generate coverage curve of each reads
pub struct SubCommandCoverage {
    #[clap(short = 'o', long = "output")]
    /// Output path (default: stdout)
    pub output: Option<std::path::PathBuf>,

    #[clap(short = 'i', long = "inputs")]
    /// Paths to the sequence files where coverage should be detect (if not set main command inputs is use)
    pub inputs: Option<Vec<std::path::PathBuf>>,
}

impl SubCommandCoverage {
    /// Get output or default value
    pub fn output(&self) -> error::Result<Box<dyn std::io::Write>> {
        self.output
            .as_ref()
            .map(|path| io::get_writer(path))
            .unwrap_or_else(|| Ok(Box::new(std::io::stdout())))
    }

    /// Get output or default value as String
    pub fn output_as_string(&self) -> String {
        match &self.output {
            Some(path) => path.to_path_buf().into_os_string().into_string().unwrap(),
            None => "stdout".to_string(),
        }
    }
}

#[derive(std::fmt::Debug, std::clone::Clone, std::marker::Copy)]
pub enum OutputFormat {
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = error::Cli;

    fn from_str(params: &str) -> Result<Self, Self::Err> {
        match params {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            _ => Err(error::Cli::OutputFormatCast {
                params: params.to_string(),
            }),
        }
    }
}

#[derive(clap::Parser, std::fmt::Debug)]
/// Detect low coverage region
pub struct SubCommandDetect {
    #[clap(short = 'f', long = "format")]
    /// Output format (default: text)
    pub format: Option<OutputFormat>,

    #[clap(short = 'o', long = "output")]
    /// Output path (default: stdout)
    pub output: Option<std::path::PathBuf>,

    #[clap(short = 'i', long = "inputs")]
    /// Paths to the sequence files where chimera should be detect (if not set main command inputs is use)
    pub inputs: Option<Vec<std::path::PathBuf>>,
}

impl SubCommandDetect {
    /// Get output or default value
    pub fn output(&self) -> error::Result<Box<dyn std::io::Write>> {
        self.output
            .as_ref()
            .map(|path| io::get_writer(path))
            .unwrap_or_else(|| Ok(Box::new(std::io::stdout())))
    }

    /// Get output or default value as String
    pub fn output_as_string(&self) -> String {
        match &self.output {
            Some(path) => path.to_path_buf().into_os_string().into_string().unwrap(),
            None => "stdout".to_string(),
        }
    }

    /// Get format or default value
    pub fn format(&self) -> OutputFormat {
        self.format.unwrap_or(OutputFormat::Text)
    }
}

#[derive(clap::Parser, std::fmt::Debug)]
/// Filter read with low coverage region
pub struct SubCommandFilter {
    #[clap(short = 'i', long = "inputs")]
    /// Paths to the sequence files where chimera should be filter (if not set main command inputs is use)
    pub inputs: Option<Vec<std::path::PathBuf>>,

    #[clap(short = 'o', long = "output")]
    /// Output path (default: stdout)
    pub outputs: Option<Vec<std::path::PathBuf>>,
}

impl SubCommandFilter {
    /// Get outputs or default value
    pub fn outputs(&self) -> error::Result<Vec<Box<dyn std::io::Write>>> {
        if let Some(paths) = &self.outputs {
            let outs: Result<Vec<_>, _> = paths
                .iter()
                .map(|path| io::get_writer(path.as_ref()))
                .collect();

            outs
        } else {
            Ok(vec![Box::new(std::io::stdout())])
        }
    }
}

#[derive(clap::Parser, std::fmt::Debug)]
/// Remove low coverage region
pub struct SubCommandClean {
    #[clap(short = 'i', long = "inputs")]
    /// Paths to the sequence files where chimera should be clean (if not set main command inputs is use)
    pub inputs: Option<Vec<std::path::PathBuf>>,

    #[clap(short = 'o', long = "output")]
    /// Output path (default: stdout)
    pub outputs: Option<Vec<std::path::PathBuf>>,
}

impl SubCommandClean {
    /// Get outputs or default value
    pub fn outputs(&self) -> error::Result<Vec<Box<dyn std::io::Write>>> {
        if let Some(paths) = &self.outputs {
            let outs: Result<Vec<_>, _> = paths
                .iter()
                .map(|path| io::get_writer(path.as_ref()))
                .collect();

            outs
        } else {
            Ok(vec![Box::new(std::io::stdout())])
        }
    }
}

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
