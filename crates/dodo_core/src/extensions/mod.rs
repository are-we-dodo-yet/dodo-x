use color_eyre::{Result, eyre::OptionExt};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::{
	ffi::OsStr,
	io::{self, Cursor},
};
use tar::Archive;

#[derive(Debug)]
pub struct ExtensionPayload {
	pub prelude: String,
	pub component: String,
	pub config: ExtensionConfig,
}

impl ExtensionPayload {
	/// # Errors
	/// Returns errors upon
	/// - Failing to read tar file
	/// - Integer overflow
	/// - Missing files in tar
	/// - Invalid strings
	#[allow(clippy::unbuffered_bytes)]
	pub fn from_tar_file(buf: &[u8]) -> Result<Self> {
		debug!("loading extension");

		let mut archive = Archive::new(Cursor::new(buf));

		let entries: io::Result<Vec<_>> = archive.entries()?.raw(true).collect();
		let mut entries = entries?;
		let mut prelude: Option<String> = None;
		let mut component: Option<String> = None;
		let mut config: Option<ExtensionConfig> = None;

		for ele in &mut entries {
			let path = ele.path()?;

			let bytes: Vec<u8> = buf
				.iter()
				.skip(
					ele.raw_file_position()
						.checked_cast()
						.ok_or_eyre("Failed to cast u64 to usize")?,
				)
				.take(ele.size().checked_cast().ok_or_eyre("Failed to cast u64 to usize")?)
				.copied()
				.collect();

			if path.file_name() == Some(OsStr::new("prelude.js")) {
				prelude = Some(String::from_utf8(bytes)?);
			} else if path.file_name() == Some(OsStr::new("component.js")) {
				component = Some(String::from_utf8(bytes)?);
			} else if path.file_name() == Some(OsStr::new("config.json")) {
				let config_text = String::from_utf8(bytes)?;
				config = Some(serde_json::from_str(&config_text)?);
			} else {
				warn!("unknown extension file {}", path.display());
			}
		}

		let prelude = prelude.ok_or_eyre("could not find prelude.js")?;
		let component = component.ok_or_eyre("could not find component.js")?;
		let config = config.ok_or_eyre("could not find config.json")?;

		debug!("loaded extension {}", config.name);

		Ok(Self { prelude, component, config })
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionConfig {
	pub name: String,
	pub description: String,
	pub id: String,
}
