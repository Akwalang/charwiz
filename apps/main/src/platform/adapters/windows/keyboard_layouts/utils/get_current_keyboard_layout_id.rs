use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn get_current_keyboard_layout_id() -> anyhow::Result<String> {
  unsafe {
    let hwnd = GetForegroundWindow();

    if hwnd.0.is_null() {
      anyhow::bail!("No foreground window");
    }

    let thread_id = GetWindowThreadProcessId(hwnd, None);

    let hkl = GetKeyboardLayout(thread_id);

    Ok(format!("{:08X}", (hkl.0 as usize) & 0x0000FFFF))
  }
}
