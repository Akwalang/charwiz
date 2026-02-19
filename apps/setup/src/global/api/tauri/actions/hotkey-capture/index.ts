import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import { KeyboardSnapshot } from "@/global/types/settings/types";

import { TauriCommand, TauriStream } from "../../enums";

enum HotkeyEventType {
  Unsubscribe = "unsubscribe",
  Snapshot = "snapshot",
} 

type StreamPayload =
  | { eventType: HotkeyEventType.Unsubscribe, listenerId: string } 
  | { eventType: HotkeyEventType.Snapshot, listenerId: string, snapshot: KeyboardSnapshot }; 

export const startHotkeyCapture = (listenerId: string): Promise<string> => invoke(TauriCommand.StartHotkeyCapture, { listenerId });
export const stopHotkeyCapture = (listenerId: string): Promise<string> => invoke(TauriCommand.StopHotkeyCapture, { listenerId });

export const retrieveHotkey = (
  onUpdate: (listenerId: string, snapshot: KeyboardSnapshot) => void,
  onClose: (listenerId: string) => void,
): Promise<() => void> => {
  const callback = ({ payload }: { payload: StreamPayload }) => {
    switch (payload.eventType) {
      case HotkeyEventType.Unsubscribe:
        return onClose(payload.listenerId);
      case HotkeyEventType.Snapshot:
        return onUpdate(payload.listenerId, payload.snapshot);
    }
  };

  return listen<StreamPayload>(TauriStream.StreamHotkeyCapture, callback);
};
