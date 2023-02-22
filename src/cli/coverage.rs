//! Command Line Interface declaration of project LCD

/* std use */

/* crate use */
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/* project use */
use crate::cli;
use crate::coverage;
use crate::error;
#[cfg(feature = "parallel")]
use crate::types;

/// SubCommand CoverageGap
#[derive(clap::Args, std::fmt::Debug)]
pub struct Coverage {
    /// Path to output, default write in stdout
    #[clap(short = 'o', long = "output")]
    output: Option<std::path::PathBuf>,
}

impl Coverage {
    /// Get output
    pub fn output(&self) -> error::Result<Box<dyn std::io::Write + std::marker::Send>> {
        match &self.output {
            None => Ok(Box::new(std::io::BufWriter::new(std::io::stdout()))),
            Some(path) => cli::create(path),
        }
    }

    #[cfg(feature = "parallel")]
    /// Main function of subcommand coverage
    pub fn main(
        &self,
        counter: &pcon::counter::Counter<pcon::CountType>,
        params: &cli::Command,
    ) -> error::Result<()> {
        let mut reader = noodles::fasta::Reader::new(params.input()?);
        let mut writer = self.output()?;

        log::info!("Start compute read coverage");

        let mut iter = reader.records();
        let mut records = Vec::with_capacity(params.record_buffer() as usize);

        let mut end = true;
        while end {
            log::info!("Start populate buffer");
            end = crate::populate_buffer(&mut iter, &mut records, params.record_buffer());
            log::info!("End populate buffer {}", records.len());

            log::info!("Start compute coverage of buffer");
            let mut name2cov: Vec<(String, types::Coverage)> = records
                .par_iter()
                .filter(|record| record.sequence().len() > counter.k() as usize)
                .map(|record| {
                    (
                        record.name().to_string(),
                        coverage::compute(&record, counter),
                    )
                })
                .collect();
            log::info!("End compute coverage of buffer");

            log::info!("Start write coverage of buffer");
            let results: Vec<_> = name2cov
                .drain(..)
                .map(|(name, coverages)| writeln!(writer, "{}: {}", name, coverages))
                .filter(Result::is_ok)
                .collect();
            log::info!("End write coverage of buffer");

            for result in results {
                if let Err(err) = result {
                    log::error!("{}", err);
                }
            }
        }
        log::info!("End compute read coverage");

        Ok(())
    }

    #[cfg(not(feature = "parallel"))]
    /// Main function of subcommand coverage
    pub fn main(
        &self,
        counter: &pcon::counter::Counter<pcon::CountType>,
        params: &cli::Command,
    ) -> error::Result<()> {
        let mut reader = noodles::fasta::Reader::new(params.input()?);
        let mut writer = self.output()?;

        log::info!("Start compute read coverage");
        let mut records = reader.records();

        while let Some(Ok(record)) = records.next() {
            if record.sequence().len() >= counter.k() as usize {
                writeln!(
                    writer,
                    "{}: {}",
                    record.name(),
                    coverage::compute(&record, counter)
                )?;
            }
        }
        log::info!("End compute read coverage");

        Ok(())
    }
}
