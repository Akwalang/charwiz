import { MainSettings } from "@/global/types/lang/settings/main";

import {
  InjectMethodEnum,
  KeyboardLayoutEnum,
  KeyboardStateCleanupEnum,
  TransformTargetEnum,
  TransformerTypeEnum,
  UserInputCleanupEnum,
} from "@/global/types/settings/enums";

export const main: MainSettings = {
  fields: {
    title: {
      name: "Назва",
    },
    description: {
      name: "Апісанне",
    },
    cmd: {
      name: "Тэкставая каманда",
    },
    keyboardSnapshot: {
      name: "Клавішы клавіятуры",
    },
    text: {
      name: "Тэкст для пераключэння",
    },
    symbol: {
      name: "Уставіць сімвал",
    },
    injectMethod: {
      name: "Метад устаўкі",
      values: {
        [InjectMethodEnum.Emulate]: "Паўтарыць дзеянні карыстальніка",
        [InjectMethodEnum.Paste]: "Уставіць",
        [InjectMethodEnum.TypeAndPaste]: "Увесці і ўставіць адсутныя сімвалы",
        [InjectMethodEnum.TypeAndSkip]: "Увесці і прапусціць адсутныя сімвалы",
      },
    },
    keyboardLayout: {
      name: "Раскладка пасля пераключэння",
      values: {
        [KeyboardLayoutEnum.Previous]: "Папярэдняя раскладка",
        [KeyboardLayoutEnum.Current]: "Бягучая раскладка",
        [KeyboardLayoutEnum.Next]: "Наступная раскладка",
      },
    },
    keyboardLayoutBefore: {
      name: "Раскладка перад выкананнем",
      values: {
        [KeyboardLayoutEnum.Previous]: "Папярэдняя раскладка",
        [KeyboardLayoutEnum.Current]: "Бягучая раскладка",
        [KeyboardLayoutEnum.Next]: "Наступная раскладка",
      },
    },
    keyboardLayoutAfter: {
      name: "Раскладка пасля выканання",
      values: {
        [KeyboardLayoutEnum.Previous]: "Папярэдняя раскладка",
        [KeyboardLayoutEnum.Current]: "Бягучая раскладка",
        [KeyboardLayoutEnum.Next]: "Наступная раскладка",
      },
    },
    keyboardStateCleanup: {
      name: "Ачыстка ўнутранага стану",
      values: {
        [KeyboardStateCleanupEnum.None]: "Не ачышчаць",
        [KeyboardStateCleanupEnum.Drop]: "Ачысціць",
      },
    },
    transformTarget: {
      name: "Мэта трансфармацыі",
      values: {
        [TransformTargetEnum.All]: "Увесь дакумент",
        [TransformTargetEnum.Clipboard]: "Буфер абмену",
        [TransformTargetEnum.Command]: "Каманда",
        [TransformTargetEnum.Events]: "Падзеі карыстальніка",
        [TransformTargetEnum.Input]: "Увод карыстальніка",
        [TransformTargetEnum.Line]: "Радок дакумента",
        [TransformTargetEnum.None]: "Нічога",
        [TransformTargetEnum.Selection]: "Выдзеленае",
        [TransformTargetEnum.Word]: "Слова",
      },
    },
    transformerType: {
      name: "Трансфарматар",
      values: {
        [TransformerTypeEnum.Native]: "Убудаваная функцыя",
        [TransformerTypeEnum.Plugin]: "Дадатковыя скрыпты",
        [TransformerTypeEnum.Static]: "Статычнае значэнне",
      },
    },
    userInputCleanup: {
      name: "Ачыстка ўводу карыстальніка",
      values: {
        [UserInputCleanupEnum.None]: "Не ачышчаць",
        [UserInputCleanupEnum.Backspace]: "Ачысціць з дапамогай клавішы Backspace",
      },
    },
  },
};
