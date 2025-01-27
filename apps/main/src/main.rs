#![windows_subsystem = "windows"]

mod constants;
mod services;

use futures::join;

use services::{Application, Tray};

#[async_std::main]
async fn main() {
  let mut application = Application::new();
  let mut tray = Tray::new();

  let _= join!(
    application.run(),
    tray.run(),
  );
}
