import { Command } from "./command.type";
import { Hotkey } from "./hotkey.type";
import { Switch } from "./switch.type";
import { Symbol } from "./symbol.type";
import { Timings } from "./timings.type";
import { Tooltip } from "./tooltip.type";

export type MainSettings = {
  timings: Timings,
  switches: Switch[],
  hotkeys: Hotkey[],
  symbols: Symbol[],
  commands: Command[],
  tooltips: Tooltip[],
};
