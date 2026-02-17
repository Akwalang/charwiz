import { CommandsLang } from "./commands";
import { HotkeysLang } from "./hotkeys";
import { PreferencesLang } from "./preferences";
import { SettingsLang } from "./settings";
import { SwitchesLang } from "./switches";

export type LangType = {
  commands: CommandsLang,
  hotkeys: HotkeysLang,
  preferences: PreferencesLang,
  settings: SettingsLang,
  switches: SwitchesLang,
};
