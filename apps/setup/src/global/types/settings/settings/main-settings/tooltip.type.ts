import { TooltipTypeEnum } from "../../enums/tooltip-type.enum";

import { CommonValues } from "./common-values.type";

export type Tooltip = {
  id: string,
  items: TooltipItem[],
};

export type TooltipItem = CommonValues & {
  settings: TooltipSettings[],
};

export type TooltipSettings = {
  type: TooltipTypeEnum,
  label: string,
};
