import { Command } from "@/global/types/settings/settings/main-settings";

export type CommandItem = {
  id: string,
  isEditing: boolean,
  current: {
    title: string,
    description: string,
    settings: Command,
  },
  editing: null | {
    title: string,
    description: string,
    settings: Command,
  },
};

export type CommandsState = {
  commands: CommandItem[],
};

export const CommandsState = (): CommandsState => ({
  commands: [],
} as const);
