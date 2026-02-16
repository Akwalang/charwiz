import { isSuperSet } from "@/utils/object";

import { CommandItem } from "../state";
import { DEFAULT_COMMAND } from "../constants";

export const isDefaultCommand = (command: CommandItem): boolean => {
  return isSuperSet(command, DEFAULT_COMMAND);
};
