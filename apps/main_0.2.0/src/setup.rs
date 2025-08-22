use rust_logger::alias;

pub fn setup() {
  alias!("$", "purple,i");
  alias!("!", "yellow");
  alias!("+", "green");
  alias!("-", "red");
  alias!("&", "cyan");
  alias!("i&", "cyan,i");
}
