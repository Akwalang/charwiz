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

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
  setup::setup();

  let local = tokio::task::LocalSet::new();

  let platform = Platform::new();
  let settings = Settings::new(&platform);

  platform.init();
  settings.init();

  let application = Application::new(&platform, &settings);

  local.run_until(async move {
    application.run().await;
  }).await;

  Ok(())
}
