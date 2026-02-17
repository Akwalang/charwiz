import {
  InjectMethodEnum,
  KeyboardLayoutEnum,
  KeyboardStateCleanupEnum,
  TransformerTypeEnum,
  TransformTargetEnum,
  UserInputCleanupEnum,
} from "@/global/types/settings/enums";

import { SwitchItem } from "./state";

export const NEW_SWITCH_TITLE = "Switch title";
export const NEW_SWITCH_DESCRIPTION = "Switch description";

export const DEFAULT_SWITCH: Omit<SwitchItem, 'id'> = {
  isEditing: false,
  current: {
    title: NEW_SWITCH_TITLE,
    description: NEW_SWITCH_DESCRIPTION,
    settings: {
      text: "",
      transformer: {
        type: TransformerTypeEnum.Native,
        value: "none",
      },
      injector: {
        target: TransformTargetEnum.Events,
        keyboardStateCleanup: KeyboardStateCleanupEnum.Drop,
        layoutBefore: KeyboardLayoutEnum.Next,
        layoutAfter: KeyboardLayoutEnum.Next,
        method: InjectMethodEnum.Emulate,
        userInputCleanup: UserInputCleanupEnum.Backspace,
      },
    },
  },
  editing: null,
};
