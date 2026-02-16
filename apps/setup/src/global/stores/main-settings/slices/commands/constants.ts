import {
  InjectMethodEnum,
  KeyboardLayoutEnum,
  KeyboardStateCleanupEnum,
  TransformerTypeEnum,
  TransformTargetEnum,
  UserInputCleanupEnum,
} from "@/global/types/settings/enums";

import { CommandItem } from "./state";

export const NEW_COMMAND_TITLE = "Command title";
export const NEW_COMMAND_DESCRIPTION = "Command description";

export const DEFAULT_COMMAND: Omit<CommandItem, 'id'> = {
  isEditing: false,
  current: {
    title: NEW_COMMAND_TITLE,
    description: NEW_COMMAND_DESCRIPTION,
    settings: {
      cmd: "",
      transformer: {
        type: TransformerTypeEnum.Native,
        value: "none",
      },
      injector: {
        target: TransformTargetEnum.None,
        keyboardStateCleanup: KeyboardStateCleanupEnum.Drop,
        layoutBefore: KeyboardLayoutEnum.Current,
        layoutAfter: KeyboardLayoutEnum.Current,
        method: InjectMethodEnum.TypeAndPaste,
        userInputCleanup: UserInputCleanupEnum.Backspace,
      },
    },
  },
  editing: null,
};
