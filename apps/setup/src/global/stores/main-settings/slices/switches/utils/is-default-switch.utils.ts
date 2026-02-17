import { isSuperSet } from "@/utils/object";

import { SwitchItem } from "../state";
import { DEFAULT_SWITCH } from "../constants";

export const isDefaultSwitch = (switch: SwitchItem): boolean => {
  return isSuperSet(switch, DEFAULT_SWITCH);
};
