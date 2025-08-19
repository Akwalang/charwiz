use std::time::Duration;
use async_std::

use windows::{
  core::w,
  Win32::{
    Foundation::{HWND, POINT},
    UI::WindowsAndMessaging::{
      BringWindowToTop, CreatePopupMenu, CreateWindowExW, DestroyMenu, GetCursorPos, SetForegroundWindow, SetWindowPos, TrackPopupMenu, HMENU, SWP_NOSIZE, SWP_NOZORDER, WINDOW_EX_STYLE, WINDOW_STYLE, WS_CAPTION, WS_OVERLAPPED, WS_SYSMENU
    },
  },
};

struct Window {
  hwnd: HWND,
}

impl Window {
  pub fn new() -> Self {
    let hwnd = Self::create_hidden_window();

    if let Err(e) = hwnd {
      panic!("Failed to create hidden window: {:?}", e);
    }

    Self {
      hwnd: hwnd.unwrap(),
    }
  }

  fn create() -> anyhow::Result<HWND> {
    unsafe {
      let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("BUTTON"),  // use button for better compatibility
        w!("Charwiz"),
        WINDOW_STYLE(WS_OVERLAPPED.0 | WS_CAPTION.0 | WS_SYSMENU.0),
        -10000, // open out from screen
        -10000, // open out from screen
        400,
        300,
        None,
        None,
        None,
        None,
      )?;

      println!("Window created successfully: {:?}", hwnd);

      Ok(hwnd)
    }
  }

  pub fn show() {
    unsafe {
      SetWindowPos(
        self.hwnd,
        None,
        0,
        0,
        1,
        1,
        SWP_NOZORDER | SWP_NOSIZE,
      );
    }
  }

  pub fn hide() {
    unsafe {
      SetWindowPos(
        self.hwnd,
        None,
        -10000,
        -10000,
        400,
        300,
        SWP_NOZORDER,
      );
    };
  }

  pub fn activate(&self) {
    unsafe {
      SetForegroundWindow(self.hwnd);
      BringWindowToTop(self.hwnd);
    };
  }
}

struct Menu {
  menu: i32,
}

impl Menu {
  pub fn new(options: Vec<>) -> Self {
    let menu = Self::create();

    if let Err(e) = menu {
      panic!("Failed to create menu: {:?}", e);
    }

    Self {
      menu: menu.unwrap()
    }
  }

  fn create() -> anyhow::Result<HMENU> {
    unsafe {
      CreatePopupMenu()
    }
  }

  fn set_options(options: Vec<>) {
    for option in options {
      unsafe {
        AppendMenuW(
          self.menu,
          MF_STRING,
          option.id,
          w!(option.label),
        );
      }
    }
  }

  fn get_selection(&self, point: POINT, hwnd: &HWND) {
    let flags = TPM_LEFTALIGN | TPM_RIGHTBUTTON | TPM_RETURNCMD;
    
    unsafe {
      let result = TrackPopupMenu(
        self.menu,
        flags,
        point.x,
        point.y,
        0,
        hwnd,
        None,
      );
    };
  }
}

impl Drop for Menu {
  fn drop(&mut self) {
    unsafe {
      DestroyMenu(self.menu);
    }
  }
}

pub struct ContextMenu {
  window: Window,
  items: Vec<MenuItem>,
}

impl ContextMenu {
  pub fn new() -> Self {
    ContextMenu {
      window: Window::new(),
      items: Vec::new(),
    }
  }

  pub fn set_items(&mut self, label: &str, action: fn()) {
    // Implementation for adding an item to the context menu
  }

  pub async fn show(&self) -> anyhow::Result<()> {
    let point = Self::get_cursor_position()?;

    println!("Cursor position: x={}, y={}", point.x, point.y);

    self.window.show();
    self.window.activate();
    
    async_std::task::sleep(Duration::from_millis(100)).await;

    



    self.window.hide();



    
    
    // Создаем контекстное меню
    let menu = CreatePopupMenu();
    match menu {
        Ok(menu) => {
            println!("Меню создано успешно");
            
            // Добавляем ваши пункты меню
            let _ = AppendMenuW(menu, MF_STRING, 1001, w!("Копировать"));
            let _ = AppendMenuW(menu, MF_STRING, 1002, w!("Вставить"));
            let _ = AppendMenuW(menu, MF_SEPARATOR, 0, None);
            let _ = AppendMenuW(menu, MF_STRING, 1003, w!("Вырезать"));
            let _ = AppendMenuW(menu, MF_STRING, 1004, w!("Свойства"));
            let _ = AppendMenuW(menu, MF_SEPARATOR, 0, None);
            let _ = AppendMenuW(menu, MF_STRING, 1005, w!("Выход"));
            
            println!("Пункты меню добавлены");
            
            // Показываем меню
            let flags = TPM_LEFTALIGN | TPM_RIGHTBUTTON | TPM_RETURNCMD;
            let result = TrackPopupMenu(
                menu,
                flags,
                point.x,
                point.y,
                0,
                self.hwnd,
                None,
            );
            
            println!("TrackPopupMenu вызван, результат: {:?}", result);
            
            if result.0 != 0 {
                let cmd_id = result.0;
                match cmd_id {
                    1001 => println!("Выбрано: Копировать"),
                    1002 => println!("Выбрано: Вставить"),
                    1003 => println!("Выбрано: Вырезать"),
                    1004 => println!("Выбрано: Свойства"),
                    1005 => {
                        println!("Выбрано: Выход");
                        running.store(false, Ordering::Relaxed);
                    }
                    _ => println!("Неизвестный пункт меню: {}", cmd_id),
                }
                
                // Сбрасываем состояние после выбора пункта
                // Это позволяет меню показываться снова
                std::thread::sleep(std::time::Duration::from_millis(100));
            } else {
                println!("Меню было закрыто без выбора");
            }
            
            // Уничтожаем меню
            let _ = DestroyMenu(menu);
        }
        Err(e) => {
            println!("ОШИБКА: Не удалось создать меню!");
            println!("Код ошибки: {:?}", e);
        }
    }
    
    // Перемещаем окно обратно за пределы экрана
  }

  fn get_cursor_position() -> anyhow::Result<POINT> {
    let mut point: POINT = POINT::default();
    
    unsafe { GetCursorPos(&mut point) }?;

    Ok(point)
  }

}
