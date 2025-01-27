// #![windows_subsystem = "windows"]

mod constants;
mod services;

use services::Application;

#[async_std::main]
async fn main() {
  Application::new().run().await;
}
