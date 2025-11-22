use rust_logger::alias;

pub fn setup() {
    setup_logger();
}

fn setup_logger() {
    alias!("$", "purple,i");
    alias!("!", "yellow");
    alias!("+", "green");
    alias!("-", "red");
    alias!("&", "cyan");
    alias!("i&", "cyan,i");
}
