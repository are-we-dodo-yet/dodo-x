use v8::V8::{initialize, initialize_platform};

/// Initializes the process-wide V8 platform.
///
/// This must be called once before creating a [`super::Runtime`].
pub fn setup_v8() {
	let platform = v8::new_default_platform(0, false).make_shared();
	initialize_platform(platform);
	initialize();
}

/// Allocate a string on a context, returns a [`Result`]
macro_rules! allocate_str {
	($string: expr, $scope: expr) => {
		v8::String::new($scope, $string)
			.ok_or_else(|| color_eyre::eyre::eyre!("V8 failed to allocate a string"))
	};
}

pub(crate) use allocate_str;
