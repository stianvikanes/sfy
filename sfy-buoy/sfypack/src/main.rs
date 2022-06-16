use anyhow::ensure;
use argh::FromArgs;
use std::path::{Path, PathBuf};
use std::ops::Deref;

use sfypack::axl;

#[derive(FromArgs)]
/// Load and print Axl package from binary collection.
struct SfyPack {
    #[argh(positional, description = "file name")]
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let pck: SfyPack = argh::from_env();
    eprintln!("Loading collection from: {:?}", pck.file);

    Ok(())
}

struct Collection {
    pub pcks: Vec<axl::AxlPacket>,
}

impl Collection {
    pub fn from_file(p: impl AsRef<Path>) -> anyhow::Result<Collection> {
        let p = p.as_ref();
        let mut b = std::fs::read(p)?;

        ensure!(
            b.len() % axl::AXL_POSTCARD_SZ == 0,
            "Collection consists of non-integer number of packages"
        );

        let n = b.len() / axl::AXL_POSTCARD_SZ;

        eprintln!("Parsing {} bytes of packages..", b.len());
        let pcks = b
            .chunks_exact_mut(axl::AXL_POSTCARD_SZ)
            .map(|p| {
                postcard::from_bytes_cobs(p).map_err(|e| anyhow::anyhow!("failed to parse package"))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Collection { pcks })
    }
}

impl Deref for Collection {
    type Target = Vec<axl::AxlPacket>;

    fn deref(&self) -> &Vec<axl::AxlPacket> {
        &self.pcks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_collection() {
        let c = Collection::from_file("tests/73.1").unwrap();
        println!("packages: {}", c.pcks.len());

        let c = Collection::from_file("tests/74.1").unwrap();
        println!("packages: {}", c.pcks.len());

        // for p in c.pcks {
        //     println!("Package: {:?}", p);
        // }
    }
}
