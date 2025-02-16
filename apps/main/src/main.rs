// #![windows_subsystem = "windows"]

mod constants;
mod services;

use crate::services::application::Application;

#[async_std::main]
async fn main() {
  Application::run().await;
}
