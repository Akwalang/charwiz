use x11::xlib;
use x11::xlib::xkb;

const XKB_USE_CORE_KBD: u32 = 0x0100;

pub fn switch_global_keyboard_layout(layout_code: &str) -> anyhow::Result<()> {
  unsafe {
    let display = xlib::XOpenDisplay(std::ptr::null());
    if display.is_null() {
      return Err(anyhow::anyhow!("Failed to open X display"));
    }

    // mapping LANGID → XKB group
    let target_group: u8 = match layout_code {
      "00000409" => 0, // en
      "00000419" => 1, // ru
      _ => {
        xlib::XCloseDisplay(display);
        return Err(anyhow::anyhow!(
          "Unknown keyboard layout code: {}",
          layout_code
        ));
      }
    };

    let status = xlib::XkbLockGroup(
      display,
      XKB_USE_CORE_KBD,
      target_group.into(),
    );

    xlib::XFlush(display);
    xlib::XCloseDisplay(display);

    if status != xlib::Success as i32 {
      return Err(anyhow::anyhow!("XkbLockGroup failed"));
    }

    Ok(())
  }
}