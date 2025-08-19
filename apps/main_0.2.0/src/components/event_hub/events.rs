use rdev::EventType;

#[derive(Debug, Clone)]
pub struct InputEvent {
  pub r#type: EventType,
}

#[derive(Debug, Clone)]
pub struct CommandEvent {
  pub command: String,
}

#[derive(Debug, Clone)]
pub struct StatusEvent {

}
