// #![windows_subsystem = "windows"]

mod setup;
mod common;
mod utils;
mod constants;

mod platform;
mod settings;

mod app;
mod components;

use app::Application;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  setup::setup();

  Application::new().run()?;

  loop {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
  }
}
