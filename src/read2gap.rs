//! A collections to store low covered region associate to a read name

/* std use */

/* crate use */

/* project use */
use crate::error::*;
use crate::*;

#[derive(std::default::Default, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
pub struct Read2Gap(pub rustc_hash::FxHashMap<String, Vec<(usize, usize)>>);

impl Read2Gap {
    pub fn json(&self) -> error::Result<String> {
        serde_json::to_string(&self.0).map_err(|error| Error::Transparent(Box::new(error)))
    }

    pub fn text(&self) -> error::Result<String> {
        let mut ret = String::new();

        for (name, gap) in self.0.iter() {
            ret.push_str(&format!("{}: {:?}\n", name, gap));
        }

        Ok(ret)
    }
}
