use tokio::task::JoinHandle;

pub struct Stream {
  pub listener_id: String,
  pub handle: JoinHandle<()>,
}
