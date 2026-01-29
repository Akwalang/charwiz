use x11::xlib;
use x11::xlib::xkb;

const XKB_USE_CORE_KBD: u32 = 0x0100;

pub fn get_current_keyboard_layout_id() -> String {
  unsafe {
    let display = xlib::XOpenDisplay(std::ptr::null());
    if display.is_null() {
      // семантически аналогично невозможности получить HWND
      return "00000000".to_string();
    }

    let mut state: xlib::XkbStateRec = std::mem::zeroed();
    let status = xlib::XkbGetState(
      display,
      XKB_USE_CORE_KBD,
      &mut state,
    );

    xlib::XCloseDisplay(display);

    if status != xlib::Success as i32 {
      return "00000000".to_string();
    }

    let group = state.group;

    // Маппинг group → pseudo LANGID
    match group {
      0 => "00000409", // en_US
      1 => "00000419", // ru_RU
      _ => "00000000",
    }
    .to_string()
  }
}