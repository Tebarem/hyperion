use std::{io::BufReader, path::PathBuf};

use anyhow::Context;
use flate2::bufread::GzDecoder;
use tar::Archive;
use tracing::info;

#[allow(
    clippy::cognitive_complexity,
    reason = "todo break up into smaller functions"
)]
pub fn get_nyc_save() -> anyhow::Result<PathBuf> {
    // $HOME/.hyperion
    let home_dir = dirs_next::home_dir().context("could not find home directory")?;

    let hyperion = home_dir.join(".hyperion");

    if !hyperion.exists() {
        // create
        info!("creating .hyperion");
        std::fs::create_dir_all(&hyperion).context("failed to create .hyperion")?;
    }

    // NewYork.tar.gz

    let new_york_dir = hyperion.join("NewYork");

    if new_york_dir.exists() {
        info!("using cached NewYork load");
    } else {
        // download
        info!("downloading NewYork.tar.gz");

        // https://github.com/andrewgazelka/maps/raw/main/NewYork.tar.gz
        let url = "https://github.com/andrewgazelka/maps/raw/main/NewYork.tar.gz";

        let response = reqwest::blocking::get(url).context("failed to get NewYork.tar.gz")?;

        info!("extracting NewYork.tar.gz");

        let decompressed = GzDecoder::new(BufReader::new(response));

        // Create a new archive from the decompressed file.
        let mut archive = Archive::new(decompressed);

        archive
            .unpack(&hyperion)
            .context("failed to unpack NewYork.tar.gz")?;
    }

    Ok(new_york_dir)
}
