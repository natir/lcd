//! A collections to store low covered region associate to a read name

/* std use */

/* crate use */

/* project use */

#[derive(std::default::Default, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
pub struct Read2Gap(pub rustc_hash::FxHashMap<String, Vec<(usize, usize)>>);
