import { isSuperSet } from "@/utils/object";

import { HotkeyItem } from "../state";
import { DEFAULT_HOTKEY } from "../constants";

export const isDefaultHotkey = (hotkey: HotkeyItem): boolean => {
  return isSuperSet(hotkey, DEFAULT_HOTKEY);
};
