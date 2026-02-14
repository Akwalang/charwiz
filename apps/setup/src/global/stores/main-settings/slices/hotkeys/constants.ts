import {
  InjectMethodEnum,
  KeyboardLayoutEnum,
  KeyboardStateCleanupEnum,
  TransformerTypeEnum,
  TransformTargetEnum,
  UserInputCleanupEnum,
} from "@/global/types/settings/enums";

import { HotkeyItem } from "./state";

import {
  NEW_HOTKEY_TITLE,
  NEW_HOTKEY_DESCRIPTION,
} from "../../constants";

export const DEFAULT_HOTKEY: Omit<HotkeyItem, 'id'> = {
  isEditing: false,
  current: {
    title: NEW_HOTKEY_TITLE,
    description: NEW_HOTKEY_DESCRIPTION,
    settings: {
      keys: {
        key: null,
        modifiers: [],
      },
      transformer: {
        type: TransformerTypeEnum.Native,
        value: "none",
      },
      injector: {
        target: TransformTargetEnum.Selection,
        keyboardStateCleanup: KeyboardStateCleanupEnum.None,
        layoutBefore: KeyboardLayoutEnum.Current,
        layoutAfter: KeyboardLayoutEnum.Current,
        method: InjectMethodEnum.TypeAndPaste,
        userInputCleanup: UserInputCleanupEnum.None,
      },
    },
  },
  editing: null,
};
