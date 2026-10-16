use color_eyre::Result;
use dodo_core::{
	components::ComponentId,
	javascript::{Runtime, setup_v8},
};

fn main() -> Result<()> {
	color_eyre::install()?;

	setup_v8();

	let rt = Runtime::new();
	let container = rt.add_container(ComponentId::UserCode)?;

	container.borrow_mut().eval(include_str!("main.js"))?;

	loop {
		rt.run_event_loop()?;
	}
}
