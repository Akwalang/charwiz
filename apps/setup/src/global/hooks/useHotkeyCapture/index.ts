import { useEffect, useState } from "react";

import {
  startHotkeyCapture,
  stopHotkeyCapture,
  retrieveHotkey,
} from "@/global/api/tauri/actions/hotkey-capture";

import { KeyboardSnapshot } from "@/global/types/settings/types";

export const useHotkeyCapture = (
  listenerId: string,
  id: string,
  path: string,
  update: (id: string, path: string, value: KeyboardSnapshot) => void,
): {
  isRecording: boolean,
  startListen: () => void,
  stopListen: () => void,
} => {
  const [isRecording, setIsRecording] = useState(false);

  useEffect((): () => void => {
    let rec = isRecording;

    rec ? startHotkeyCapture(listenerId) : stopHotkeyCapture(listenerId);

    return () => rec && stopHotkeyCapture(listenerId);
  }, [listenerId, isRecording]);

  useEffect(() => {
    if (!isRecording) return;

    let promise = retrieveHotkey(
      (lid, snapshot) => lid === listenerId && update(id, path, snapshot),
      (lid) => lid === listenerId && setIsRecording(false),
    );
    
    return () => {
      promise.then((stopStream) => stopStream());
    };
  }, [listenerId, id, path, update, isRecording]);

  return {
    isRecording,
    startListen: () => setIsRecording(true),
    stopListen: () => setIsRecording(false),
  };
};
