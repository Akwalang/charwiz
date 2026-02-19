import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import { KeyboardSnapshot } from "@/global/types/settings/types";

export const startHotkeyCapture = async () => console.log('>>>', await invoke("start_hotkey_capture"));
export const stopHotkeyCapture = async () => console.log('>>>', await invoke("stop_hotkey_capture"));

export const retrieveHotkey = async (
  callback: (snapshot: KeyboardSnapshot) => void,
): Promise<() => void> => {
  const unListen = await listen<KeyboardSnapshot>("stream_hotkey_capture", (event) => {
    callback(event.payload);
  });

  return unListen;
};
