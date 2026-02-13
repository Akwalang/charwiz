import * as KeyCode from 'keycode-js';
import { useEffect, useCallback } from 'react';

type Flags = { alt?: boolean, ctrl?: boolean, shift?: boolean };
type Callback = (event: WindowEventMap['keydown']) => void;

function useHotkey(key: string, flags: Flags, callback: Callback): void {
  const listener: Callback = useCallback((event) => {
    event.code === key
    && event.altKey === (flags.alt ?? false)
    && event.ctrlKey === (flags.ctrl ?? false)
    && event.shiftKey === (flags.shift ?? false)
    && callback(event);
  }, [key, flags, callback]);

  useEffect(() => {
    window.addEventListener('keydown', listener, false);

    return () => {
      window.removeEventListener('keydown', listener);
    };
  }, [listener]);
}

export { KeyCode as KeyCode };
export { useHotkey as useHotkey };
