use tokio::sync::broadcast::{channel, Receiver, Sender};

use zero_cost_logger::*;

use crate::common::events::{InputEvent, CommandEvent};

pub struct EventHub {
  input_tx: Sender<InputEvent>,
  command_tx: Sender<CommandEvent>,
}

impl EventHub {
  pub fn new() -> Self {
    let (input_tx, _rx_drop) = channel(16);
    let (command_tx, _rx_drop) = channel(16);

    Self { input_tx, command_tx }
  }

  #[inline]
  fn publish<T: Clone>(tx: &Sender<T>, ev: T, chan: &str) -> anyhow::Result<()> {
    if let Err(_) = tx.send(ev) {
      warn!("<$>Event Hub</>: No subscribers for \"<&>{}</>\"; dropping event", chan);
    }

    Ok(())
  }

  pub fn publish_input(&self, ev: InputEvent) -> anyhow::Result<()> {
    Self::publish(&self.input_tx, ev, "input")
  }

  pub fn publish_command(&self, ev: CommandEvent) -> anyhow::Result<()> {
    Self::publish(&self.command_tx, ev, "command")
  }

  pub fn input_stream(&self) -> Receiver<InputEvent> {
    self.input_tx.subscribe()
  }

  pub fn command_stream(&self) -> Receiver<CommandEvent> {
    self.command_tx.subscribe()
  }
}
