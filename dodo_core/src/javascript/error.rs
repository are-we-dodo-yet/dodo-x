use thiserror::Error;

use crate::components::ComponentId;

/// Errors raised while creating or evaluating a V8 container.
#[derive(Error, Debug)]
pub enum RuntimeError {
	#[error("V8 failed to allocate something")]
	V8AllocationFailed,

	#[error("V8 could not compile some JavaScript")]
	V8CompilationFailed,

	#[error("V8 could not serialize a value to a string")]
	V8ToStringFailed,

	#[error("V8 could not bind a native Rust function as a JavaScript function")]
	V8FunctionBindingFailed,
	
	#[error("There already exists a container with the component ID {0}")]
	DuplicateComponentId(ComponentId),
}
