use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::Globalization::*;

use super::super::KeyboardLayout;

const MAX_LAYOUTS_COUNT: usize = 16;
const MAX_LANG_NAME_SIZE: usize = 85;

pub fn get_keyboard_layouts() -> Vec<KeyboardLayout> {
  let mut layouts = vec![HKL(std::ptr::null_mut()); MAX_LAYOUTS_COUNT];
  let count = unsafe { GetKeyboardLayoutList(Some(&mut layouts)) };

  if count == 0 { return vec![]; }

  let mut result = Vec::new();

  for &hkl in layouts.iter().take(count as usize) {
    let lang_id = (hkl.0 as usize & 0xFFFF) as u32;

    let mut buffer = [0u16; MAX_LANG_NAME_SIZE];
    let len = unsafe {
      GetLocaleInfoW(lang_id, LOCALE_SNAME, Some(&mut buffer))
    };

    let layout_name = if len > 0 {
      OsString::from_wide(&buffer[..len as usize - 1]).to_string_lossy().into_owned()
    } else {
      format!("Unknown ({:08X})", lang_id)
    };

    let layout_id = format!("{:08X}", (hkl.0 as usize) & 0x0000FFFF);

    result.push(KeyboardLayout::new(layout_id, layout_name));
  }

  result
}
