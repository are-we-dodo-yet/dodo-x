use std::{cell::RefCell, rc::Rc};

use color_eyre::Result;
use v8::{Context, ContextOptions, CreateParams, Global, Isolate, OwnedIsolate};

use crate::{components::ComponentId, javascript::RuntimeError};

use super::Container;

/// Owns the V8 isolate and the containers created inside it.
pub struct Runtime<'rt> {
	pub(crate) isolate: RefCell<OwnedIsolate>,
	pub(crate) containers: RefCell<Vec<Rc<RefCell<Container<'rt>>>>>,
}

impl<'rt> Runtime<'rt> {
	/// Creates a runtime with a new V8 isolate.
	#[must_use]
	pub fn new() -> Self {
		let isolate = Isolate::new(CreateParams::default());

		Self { isolate: RefCell::new(isolate), containers: RefCell::new(Vec::new()) }
	}

	/// Creates and retains a JavaScript container associated with this runtime.
	///
	/// # Errors
	/// Errors upon failed allocation or dropped runtime
	/// Errors upon isolate overborrowing
	pub fn add_container(&'rt self, id: ComponentId) -> Result<Rc<RefCell<Container<'rt>>>> {
		if self.containers.borrow().iter().any(|x| x.borrow().id == id) {
			return Err(RuntimeError::DuplicateComponentId(id).into());
		}

		let container = {
			let mut borrowed_isolate = self.isolate.try_borrow_mut()?;
			v8::scope!(let scope, &mut *borrowed_isolate);

			let context = Context::new(scope, ContextOptions::default());

			Rc::new(RefCell::new(Container {
				runtime: self,
				context: Global::new(scope, context),
				id,
			}))
		};

		container.borrow_mut().init()?;

		self.containers.borrow_mut().push(container.clone());

		Ok(container)
	}
}

impl Default for Runtime<'_> {
	fn default() -> Self {
		Self::new()
	}
}
