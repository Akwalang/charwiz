import { ClipboardHotkeys } from "./clipboard-hotkeys.type";

import { KeyboardSnapshot } from "../../types/keyboard-snapshot.typs";

export type PlatformSettings = {
  clipboardHotkeys: ClipboardHotkeys,
  bannedHotkeys: KeyboardSnapshot[],
  switchKeyboardLayoutHotkeys: KeyboardSnapshot[],
  stackBrakeHotkeys: KeyboardSnapshot[],
};
