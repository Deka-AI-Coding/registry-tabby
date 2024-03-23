use crate::model::Model;
use anyhow::Result;

mod v2;

pub fn parse_model(name: &str, tag: &str) -> Result<Model> {
    futures::executor::block_on(v2::parse_model(name, tag))
}
