use std::ffi::CString;

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::*;
use windows::core::PCSTR;

pub fn switch_global_keyboard_layout(layout_code: &str) -> anyhow::Result<()> {
  unsafe {
    let layout_cstr = CString::new(layout_code).expect("CString conversion failed");
    let layout_pcstr = PCSTR(layout_cstr.as_ptr() as *const u8);

    let hkl = LoadKeyboardLayoutA(layout_pcstr, KLF_ACTIVATE);

    let Ok(hkl) = hkl else {
      anyhow::bail!("Failed to load keyboard layout");
    };

    PostMessageA(
      Some(HWND_BROADCAST),
      WM_INPUTLANGCHANGEREQUEST,
      WPARAM(0),
      LPARAM(hkl.0 as isize),
    )?;

    Ok(())
  }
}
