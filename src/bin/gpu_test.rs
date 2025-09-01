use std::env;
use env_logger::Env;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;
use barnes_hut::gui::app::App;

pub fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let env = Env::default()
            .filter_or("LOG_LEVEL", "info")
            .write_style_or("LOG_STYLE", "always");

        env_logger::init_from_env(env);
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    event_loop.run_app(&mut app)?;

    Ok(())
}


fn main() {
    run().expect("run() failed");
}