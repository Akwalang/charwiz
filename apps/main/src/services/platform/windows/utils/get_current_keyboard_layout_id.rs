use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn get_current_keyboard_layout_id() -> String {
  unsafe {
    let hwnd = GetForegroundWindow();

    if hwnd.0.is_null() {
      panic!("GetForegroundWindow failed");
    }

    let thread_id = GetWindowThreadProcessId(hwnd, None);

    let hkl = GetKeyboardLayout(thread_id);

    format!("{:08X}", (hkl.0 as usize) & 0x0000FFFF)
  }
}
