use std::fmt::{self, Display};

/// Identifies an intra-process communication module
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComponentId {
	/// Identifies the singular primary user-thread JavaScript instance, at
	/// least, until we introduce webworkers
	UserCode,

	/// In reality, the Dodo API component does not exist, however, it allows us
	/// to reuse our existing mechanisms for communication, minimising the
	/// amount of V8-Rust bridge code we have to write.
	DodoApi,

	/// Represents a user-provided or a runtime-shipped extenson, increasing the
	/// features available in the runtime.
	Extension(String),
}

impl Display for ComponentId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::UserCode => f.write_str("user-defined code"),
			Self::DodoApi => f.write_str("dodo internal API"),
			Self::Extension(name) => f.write_str(&format!("extension '{name}'")),
		}
	}
}
