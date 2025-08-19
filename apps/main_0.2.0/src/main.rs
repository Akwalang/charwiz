// #![windows_subsystem = "windows"]

mod app;
mod components;

mod platform;
mod settings;

mod utils;
mod constants;

use app::Application;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  Application::new().run()?;

  loop {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
  }
}
