use rust_logger::*;

use async_broadcast::{broadcast, InactiveReceiver, Receiver, Sender, TrySendError};

use crate::components::event_hub::{InputEvent, CommandEvent, StatusEvent};

pub struct EventHub {
  input_cast: (Sender<InputEvent>, InactiveReceiver<InputEvent>),
  command_cast: (Sender<CommandEvent>, InactiveReceiver<CommandEvent>),
  status_cast: (Sender<StatusEvent>, InactiveReceiver<StatusEvent>),
}

impl EventHub {
  pub fn new() -> Self {
    let (input_tx, input_rx) = broadcast(8);
    let (command_tx, command_rx) = broadcast(8);
    let (status_tx, status_rx) = broadcast(8);

    EventHub {
      input_cast: (input_tx, input_rx.deactivate()),
      command_cast: (command_tx, command_rx.deactivate()),
      status_cast: (status_tx, status_rx.deactivate()),
    }
  }

  fn publish<T: Clone>(sender: &Sender<T>, event: T, channel_name: &str) -> anyhow::Result<()> {
    match sender.try_broadcast(event) {
      Ok(_) => Ok(()),
      Err(TrySendError::Inactive(_msg)) => Ok(()),
      Err(TrySendError::Full(_msg)) => {
        error!("<$>EventHub</>: Channel \"<&>{}</>\" is full, dropping event", channel_name);
        anyhow::bail!("Channel full")
      },
      Err(TrySendError::Closed(_msg)) => {
        error!("<$>EventHub</>: Channel \"<&>{}</>\" is closed, dropping event", channel_name);
        anyhow::bail!("Channel closed")
      },
    }
  }

  pub fn publish_input(&self, event: InputEvent) -> anyhow::Result<()> {
    Self::publish(&self.input_cast.0, event, "input")
  }
  
  pub fn publish_command(&self, command: CommandEvent) -> anyhow::Result<()> {
    Self::publish(&self.command_cast.0, command, "command")
  }

  pub fn publish_status(&self, status: StatusEvent) -> anyhow::Result<()> {
    Self::publish(&self.status_cast.0, status, "status")
  }

  pub fn input_stream(&self) -> Receiver<InputEvent> {
    self.input_cast.0.new_receiver()
  }
  
  pub fn command_stream(&self) -> Receiver<CommandEvent> {
    self.command_cast.0.new_receiver()
  }

  pub fn status_stream(&self) -> Receiver<StatusEvent> {
    self.status_cast.0.new_receiver()
  }
}
