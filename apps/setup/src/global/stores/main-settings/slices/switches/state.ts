import { Switch } from "@/global/types/settings/settings/main-settings";

export type SwitchItem = {
  id: string,
  isEditing: boolean,
  current: {
    title: string,
    description: string,
    settings: Switch,
  },
  editing: null | {
    title: string,
    description: string,
    settings: Switch,
  },
};

export type SwitchesState = {
  switches: SwitchItem[],
};

export const SwitchesState = (): SwitchesState => ({
  switches: [],
} as const);
