//! Perform detection of coverage gap

/* std use */

/* crate use */
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/* project use */
use crate::types;

#[cfg(not(feature = "parallel"))]
/// Compute coverage of read assume read is larger than k
pub fn compute(
    record: &noodles::fasta::Record,
    counter: &pcon::counter::Counter<pcon::CountType>,
) -> types::Coverage {
    let mut coverage = Vec::with_capacity(record.sequence().len() - counter.k() as usize + 1);

    for kmer in cocktail::tokenizer::Canonical::new(record.sequence().as_ref(), counter.k()) {
        coverage.push(counter.get(kmer));
    }

    types::Coverage(coverage)
}

#[cfg(feature = "parallel")]
/// Compute coverage of read assume read is larger than k
pub fn compute(
    record: &noodles::fasta::Record,
    counter: &pcon::counter::Counter<pcon::CountType>,
) -> types::Coverage {
    types::Coverage(
        cocktail::tokenizer::Canonical::new(record.sequence().as_ref(), counter.k())
            .par_bridge()
            .map(|kmer| counter.get(kmer))
            .collect(),
    )
}
