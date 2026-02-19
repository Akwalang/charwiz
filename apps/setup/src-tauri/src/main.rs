// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod components;

mod app;

use app::Application;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
  let local = tokio::task::LocalSet::new();

  let application = Application::new();

  local.run_until(async move {
    let _ = application.run().await;
  }).await;

  Ok(())
}
