//! Define some usefull type

/* std use */

/* crate use */
use itertools::Itertools;
#[cfg(feature = "json")]
use serde_json;

/* project use */
use crate::error;

/// Type alias for coverage
pub struct Coverage(pub Vec<pcon::CountTypeNoAtomic>);

impl std::fmt::Display for Coverage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|x| x.to_string()).join(","))
    }
}

/// Struct that associate reads id and coverage
pub struct Read2Coverage(Vec<(String, Coverage)>);

/// Struct to associate a read id his gap
#[derive(std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Read2Gap(pub rustc_hash::FxHashMap<String, Vec<(usize, usize)>>);

impl Read2Gap {
    #[cfg(feature = "json")]
    /// Convert Read2Gap instance in json format
    pub fn json(&self) -> error::Result<String> {
        Ok(serde_json::to_string(&self.0)?)
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
