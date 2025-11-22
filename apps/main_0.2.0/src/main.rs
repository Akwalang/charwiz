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

use platform::Platform;
use settings::Settings;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  setup::setup();

  let platform = Platform::new();
  let settings = Settings::new(&platform);

  platform.init();
  settings.init();

  let application = Application::new(&platform, &settings);

  application.run()?;

  loop {
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
  }
}
