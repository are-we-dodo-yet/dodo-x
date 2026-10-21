use color_eyre::Result;
use dodo_core::{
	components::ComponentId,
	extensions::ExtensionPayload,
	javascript::{Runtime, setup_v8},
};
use log::LevelFilter;

fn main() -> Result<()> {
	{
		let mut clog = colog::default_builder();
		clog.filter(None, LevelFilter::Trace);
		clog.init();
	}

	color_eyre::install()?;

	let console = ExtensionPayload::from_tar_file(include_bytes!("../bundled_exts/console.ddx"))?;

	setup_v8();

	let rt = Runtime::new();

	rt.load_extension(console)?;

	let user_code = rt.add_container(ComponentId::UserCode)?;

	user_code.borrow_mut().load_prelude()?;

	user_code.borrow_mut().eval(include_str!("main.js"))?;

	loop {
		rt.run_event_loop()?;
	}
}
