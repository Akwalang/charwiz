#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::Platform;

#[cfg(target_os = "linux")]
mod linux_x11;
#[cfg(target_os = "linux")]
pub use linux_x11::Platform;