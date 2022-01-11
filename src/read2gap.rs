//! A collections to store low covered region associate to a read name

/* std use */
use itertools::Itertools;

/* crate use */

/* project use */
use crate::error::*;
use crate::*;

/// Struct to associate a read id his gap
#[derive(std::default::Default, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
pub struct Read2Gap(pub rustc_hash::FxHashMap<String, Vec<(usize, usize)>>);

impl Read2Gap {
    /// Convert Read2Gap instance in json format
    pub fn json(&self) -> error::Result<String> {
        serde_json::to_string(&self.0).map_err(|error| Error::Transparent(Box::new(error)))
    }

    /// Convert Read2Gap instance in human text format
    pub fn text(&self) -> error::Result<String> {
        Ok(self
            .0
            .iter()
            .map(|(n, g)| {
                format!(
                    "{}: {}",
                    n,
                    g.iter().map(|x| format!("({}, {})", x.0, x.1)).join(",")
                )
            })
            .join("\n"))
    }
}
