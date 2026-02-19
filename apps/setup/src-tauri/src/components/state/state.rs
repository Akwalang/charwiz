use uuid::Uuid;

pub struct State {
  pub name: String,
  pub hotkey_listener_id: Option<String>,
}

impl State {
  pub fn new() -> Self {
    State {
      name: "Charwiz".to_owned(),
      hotkey_listener_id: None,
    }
  }
}
