//! Run detection of low coverage region

/* std use */

/* crate use */
use rayon::prelude::*;
use ringbuffer::{RingBufferExt, RingBufferWrite};

/* project use */
use crate::*;

pub fn detect(
    count: pcon::counter::Counter,
    main_params: &cli::Command,
    sub_params: &cli::SubCommandDetect,
) -> error::Result<()> {
    log::debug!("Run detect with params: {:?} {:?}", main_params, sub_params);

    for input in main_params.inputs.iter() {
        let mut reader =
            noodles::fasta::Reader::new(std::io::BufReader::new(io::get_reader(input)?));
        let mut records_iterator = reader.records();

        let mut records: Vec<noodles::fasta::Record> =
            Vec::with_capacity(main_params.buffer_length());

        let mut end = false;
        while !end {
            for i in 0..main_params.buffer_length() {
                if let Some(Ok(record)) = records_iterator.next() {
                    records.push(record);
                } else {
                    end = true;
                    records.truncate(i);
                    break;
                }
            }

            log::info!("Buffer len: {}", records.len());

            records.par_iter().for_each(|record| {
                if record.sequence().len() >= main_params.kmer_size as usize {
                    let tokenizer = cocktail::tokenizer::Canonical::new(
                        record.sequence().as_ref(),
                        main_params.kmer_size,
                    );

                    let mut debug = String::new();
                    let mut average: f64 = 0.0;
                    let mut ring: ringbuffer::ConstGenericRingBuffer<u8, 32> =
                        ringbuffer::ConstGenericRingBuffer::new();
                    for (i, canonical) in tokenizer.enumerate() {
                        let coverage = count.get_canonic(canonical);

                        ring.push(coverage);
                        average += (coverage as f64 - average) / (1 + i) as f64;

                        debug.push_str(&format!(
                            "{};{};{:?};{};{}\n",
                            coverage,
                            average,
                            ring.to_vec(),
                            ring.iter().map(|x| *x as f64).sum::<f64>(),
                            ring.iter().map(|x| *x as f64).sum::<f64>() / 32.0
                        ));
                    }

                    panic!("{} {}", record.definition().name(), debug);
                }
            });
        }
    }

    Ok(())
}
