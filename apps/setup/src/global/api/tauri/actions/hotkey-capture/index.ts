import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import { KeyboardSnapshot } from "@/global/types/settings/types";

import { TauriCommand, TauriStream } from "../../enums";

export const startHotkeyCapture = (listenerId: string): Promise<string> => invoke(TauriCommand.StartHotkeyCapture, { listenerId });
export const stopHotkeyCapture = (listenerId: string): Promise<string> => invoke(TauriCommand.StopHotkeyCapture, { listenerId });

export const retrieveHotkey = async (
  callback: (snapshot: KeyboardSnapshot) => void,
): Promise<() => void> => {
  const unListen = await listen<KeyboardSnapshot>(TauriStream.StreamHotkeyCapture, (event) => {
    console.log('event.payload =>', JSON.stringify(event.payload, null, 2));
    callback(event.payload);
  });

  return unListen;
};
