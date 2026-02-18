import { CommandsLang } from "./commands";
import { HotkeysLang } from "./hotkeys";
import { MenuLang } from "./menu";
import { PreferencesLang } from "./preferences";
import { SettingsLang } from "./settings";
import { SwitchesLang } from "./switches";
import { SymbolsLang } from "./symbols";

export type LangType = {
  commands: CommandsLang,
  hotkeys: HotkeysLang,
  menu: MenuLang,
  preferences: PreferencesLang,
  settings: SettingsLang,
  switches: SwitchesLang,
  symbols: SymbolsLang,
};
