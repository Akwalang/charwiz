import { Hotkey } from "@/global/types/settings/settings/main-settings";

export type HotkeyItem = {
  id: string,
  isEditing: boolean,
  current: {
    title: string,
    description: string,
    settings: Hotkey,
  },
  editing: null | {
    title: string,
    description: string,
    settings: Hotkey,
  },
};

export type HotkeysState = {
  hotkeys: HotkeyItem[],
};

export const HotkeysState = (): HotkeysState => ({
  hotkeys: [],
} as const);
