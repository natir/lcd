//! Run detection of low coverage region

/* std use */

/* crate use */

/* project use */
use crate::error::*;
use crate::*;

pub fn count_kmer(
    kmer_size: u8,
    inputs: &[std::path::PathBuf],
    buffer_length: usize,
) -> error::Result<pcon::counter::Counter> {
    log::info!("Start of kmer count");

    log::info!("Structure initialization");
    let mut counter = pcon::counter::Counter::new(kmer_size);
    log::info!("End of structure initialization");

    for path in inputs {
        log::info!("Start of kmer count of the file {}", path.display());
        let reader = get_reader(path)?;

        counter.count_fasta(reader, buffer_length);

        log::info!("End of kmer count of the file {}", path.display());
    }

    Ok(counter)
}

pub fn get_reader(path: &std::path::Path) -> error::Result<Box<dyn std::io::Read>> {
    Ok(
        niffler::get_reader(Box::new(std::fs::File::open(path).map_err(|error| {
            Error::IO(IO::CantOpenFile {
                path: path.to_path_buf().into_os_string().into_string().unwrap(),
                error,
            })
        })?))
        .map_err(|error| Error::Transparent(Box::new(error)))?
        .0,
    )
}

pub fn get_writer(path: &std::path::Path) -> error::Result<Box<dyn std::io::Write>> {
    Ok(Box::new(std::fs::File::create(path).map_err(|error| {
        Error::IO(IO::CantOpenFile {
            path: path.to_path_buf().into_os_string().into_string().unwrap(),
            error,
        })
    })?))
}
