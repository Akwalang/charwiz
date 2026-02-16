import { CommandsLang } from "./commands";
import { HotkeysLang } from "./hotkeys";
import { PreferencesLang } from "./preferences";
import { SettingsLang } from "./settings";

export type LangType = {
  commands: CommandsLang,
  hotkeys: HotkeysLang,
  preferences: PreferencesLang,
  settings: SettingsLang,
};
