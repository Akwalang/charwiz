use tray_item::{TrayItem, IconSource};
use async_std::channel;

use crate::constants::APP_NAME;

use crate::services::tray::enums::TrayAction;
use crate::services::tray::structs::MenuItem;

pub struct Tray {
  tray: TrayItem,
  sender: channel::Sender<TrayAction>,
  receiver: channel::Receiver<TrayAction>,
}

impl Tray {
  pub fn new() -> Tray {
    let mut tray = Self::create_tray();

    let (sender, receiver) = channel::unbounded::<TrayAction>();

    Self::add_menu_items(&mut tray, &sender);

    Tray { tray, sender, receiver }
  }

  pub async fn get_input(&self) -> TrayAction {
    loop {
      if let Ok(event) = self.receiver.recv().await {
        return event;
      }
    }
  }

  fn create_tray() -> TrayItem {
    let mut tray = TrayItem::new(APP_NAME, IconSource::Resource("MY_APP_ICON")).unwrap();

    tray.add_label(APP_NAME).unwrap();

    tray
  }

  fn add_menu_items(tray: &mut TrayItem, sender: &channel::Sender<TrayAction>) {
    let items = vec![
      MenuItem::new("Reload", TrayAction::Reload),
      MenuItem::new("Exit", TrayAction::Exit),
    ];

    for item in items {
      let sender_clone = sender.clone();

      let callback = move || {
        sender_clone.send_blocking(item.action.clone()).unwrap();
      };

      if let Err(_) = tray.add_menu_item(item.name, callback) {
        println!("Can't add menu item: {}", item.name);
      }
    }
  }
}
