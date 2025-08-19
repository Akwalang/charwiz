// #![windows_subsystem = "windows"]

mod constants;

mod app;
mod event_hub;

mod platform;
mod settings;

mod controllers;
mod detectors;
mod executor;

mod utils;

use app::Application;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  Application::new().run()?;

  loop {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
  }
}
