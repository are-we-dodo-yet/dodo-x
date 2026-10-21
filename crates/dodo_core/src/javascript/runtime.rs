use std::{cell::RefCell, rc::Rc};

use color_eyre::{Result, eyre::eyre};
use v8::{Context, ContextOptions, CreateParams, Global, Isolate, OwnedIsolate};

use crate::{components::ComponentId, extensions::ExtensionPayload};

use super::Container;

/// Owns the V8 isolate and the containers created inside it.
pub struct Runtime<'rt> {
	pub(crate) isolate: RefCell<OwnedIsolate>,
	pub(crate) containers: RefCell<Vec<Rc<RefCell<Container<'rt>>>>>,
	pub(crate) extensions: RefCell<Vec<ExtensionPayload>>,
}

impl<'rt> Runtime<'rt> {
	/// Creates a runtime with a new V8 isolate.
	#[must_use]
	pub fn new() -> Self {
		let isolate = Isolate::new(CreateParams::default());

		Self {
			isolate: RefCell::new(isolate),
			containers: RefCell::new(Vec::new()),
			extensions: RefCell::new(Vec::new()),
		}
	}

	/// Creates and retains a JavaScript container associated with this runtime.
	///
	/// # Errors
	/// Errors upon failed allocation or dropped runtime
	/// Errors upon isolate overborrowing
	pub fn add_container(&'rt self, id: ComponentId) -> Result<Rc<RefCell<Container<'rt>>>> {
		if self.containers.borrow().iter().any(|x| x.borrow().id == id) {
			return Err(eyre!("There already exists a component with the id {id}"));
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

	/// Run a tick of the event loop on every container
	///
	/// # Errors
	/// Throws an error when
	/// - There is no object key of global called "extensionsRuntimeBridge"
	/// - There is no function key of extensionsRuntimeBridge called
	///   "runEventLoopTick"
	/// - v8 allocation fails
	/// - A container is being borrowed
	/// - The runtime isolate is being borrowed
	pub fn run_event_loop(&self) -> Result<()> {
		for container in self.containers.borrow().iter() {
			container.try_borrow_mut()?.run_event_loop()?;
		}
		Ok(())
	}

	/// # Errors
	/// Throws an error when
	/// - Container fails to initalize
	pub fn load_extension(&'rt self, ext: ExtensionPayload) -> Result<()> {
		// Spin up the component
		let container = self.add_container(ComponentId::Extension(ext.config.name.clone()))?;

		container.borrow_mut().eval(&ext.component)?;

		self.extensions.borrow_mut().push(ext);

		Ok(())
	}
}

impl Default for Runtime<'_> {
	fn default() -> Self {
		Self::new()
	}
}
