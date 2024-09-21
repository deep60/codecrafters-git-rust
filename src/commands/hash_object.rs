use anyhow::Context;
use std::path::Path;
use crate::objects::{self, Object};



pub(crate) fn invoke(write: bool, file: &Path) -> anyhow::Result<()> {
    let object = Object::blob_from_file(file).context("open blob input file")?;
        let hash = if write {
            object.write_to_objects().context("stream file into blob object")?
        } else {
            object.write(std::io::sink()).context("stream file into blob object")?
        };
        println!("{}", hex::encode(hash));

        Ok(())
}