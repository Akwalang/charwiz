use std::collections::HashSet;
use std::sync::OnceLock;

use rdev::Key;

static BANNED_HOTKEYS: OnceLock<Vec<HashSet<Key>>> = OnceLock::new();

pub fn get_banned_hotkeys() -> &'static Vec<HashSet<Key>> {
  BANNED_HOTKEYS.get_or_init(|| {
    let mut vec: Vec<HashSet<Key>> = Vec::new();

    vec.push(create_screen_lock_hotkey());
    
    vec
  })
}

fn create_screen_lock_hotkey() -> HashSet<Key> {
  let mut set = HashSet::new();
  
  set.insert(Key::MetaLeft);
  set.insert(Key::KeyL);

  set
}
