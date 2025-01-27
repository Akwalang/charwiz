use std::time::Duration;

use tray_item::{TrayItem, IconSource};
use async_std::{channel, task};

use crate::constants::APP_NAME;

use super::structs::MenuItem;
use super::enums::TrayAction;

pub struct Tray {
  tray: TrayItem,
}

impl Tray {
  pub fn new() -> Tray {
    let items = vec![
      MenuItem::new("Exit", TrayAction::Exit),
    ];

    let tray = Tray::create_tray(items);

    Tray { tray }
  }

  fn create_tray(items: Vec<MenuItem>) -> TrayItem {
    let mut tray = TrayItem::new(APP_NAME, IconSource::Resource("MY_APP_ICON")).unwrap();

    let (sender, receiver) = channel::unbounded::<TrayAction>();

    tray.add_label(APP_NAME).unwrap();

    for item in items {
      let sender_clone = sender.clone();

      let callback = move || {
        sender_clone.send_blocking(item.action.clone()).unwrap();
      };

      if let Err(_) = tray.add_menu_item(item.name, callback) {
        println!("Can't add menu item: {}", item.name);
      }
    }

    task::spawn(async move {
      while let Ok(event) = receiver.recv().await {
        match event {
          TrayAction::Exit => {
            println!("Application is closing...");
            std::process::exit(0);
          }
        }
      }
    });

    tray
  }
}
