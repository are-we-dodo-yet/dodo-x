use color_eyre::Result;
use v8::{Context, ContextScope, Function, Global, Local, Object, Script};

use crate::{
	components::ComponentId,
	javascript::{Runtime, v8::allocate_str},
};

use super::error::RuntimeError;

/// A JavaScript execution context owned by a [`super::Runtime`].
pub struct Container<'rt> {
	/// A reference to the containing runtime of this container
	pub(crate) runtime: &'rt Runtime<'rt>,

	/// The persistent V8 handle for this container's context.
	pub context: Global<Context>,

	/// The component that this container represents
	pub id: ComponentId,
}

impl Container<'_> {
	/// Evaluates JavaScript source in this container's context.
	///
	/// # Errors
	/// Errors upon allocation failure
	pub fn eval(&self, source: &str) -> Result<()> {
		let mut borrowed_isolate = self.runtime.isolate.borrow_mut();
		v8::scope!(let handle_scope, &mut *borrowed_isolate);

		// Get a handle to the context
		let context = Local::new(handle_scope, &self.context);

		// Ensure that V8 data lasts for the duration of this function
		let scope = &ContextScope::new(handle_scope, context);

		// Allocate the code text
		let code = allocate_str!(source, scope)?;

		// Compile the script
		let script = Script::compile(scope, code, None);
		let script = script.ok_or(RuntimeError::V8CompilationFailed)?;

		// Run the script
		if let Some(val) = script.run(scope) {
			println!(
				"{}",
				val.to_string(scope)
					.ok_or(RuntimeError::V8ToStringFailed)?
					.to_rust_string_lossy(scope)
			);
		} else {
			println!("Script returned nothing.");
		}

		Ok(())
	}

	/// # Errors
	/// Errors upon allocation failure or dropped runtime
	/// Errors upon isolate overborrowing
	pub fn init(&mut self) -> Result<()> {
		let mut borrowed_isolate = self.runtime.isolate.try_borrow_mut()?;
		v8::scope_with_context!(scope, &mut *borrowed_isolate, &self.context);

		let context = Local::new(scope, &self.context);

		let api_object = Object::new(scope);

		let post_message_js_func = Function::builder(rt_post_message_impl).build(scope);

		let send_message = post_message_js_func.ok_or(RuntimeError::V8FunctionBindingFailed)?;

		api_object.set(scope, allocate_str!("postMessage", scope)?.into(), send_message.into());

		context.global(scope).set(
			scope,
			allocate_str!("extensionsRuntime", scope)?.into(),
			api_object.into(),
		);

		Ok(())
	}
}

#[allow(clippy::needless_pass_by_value)] // This requires a very specific signature
fn rt_post_message_impl(
	scope: &mut v8::PinScope<'_, '_>,
	args: v8::FunctionCallbackArguments<'_>,
	_rv: v8::ReturnValue,
) {
	if let Ok(str) = args.get(0).try_cast::<v8::String>() {
		println!("recieved message {}", str.to_rust_string_lossy(scope));
	} else {
		println!("failed to print");
	}
}
