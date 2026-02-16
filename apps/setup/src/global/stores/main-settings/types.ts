import { CommonValues } from "@/global/types/settings/settings/main-settings";

export type CommonSettings = {
  id: string,
  isEditing: boolean,
  current: {
    title: string,
    description: string,
    settings: CommonValues,
  },
  editing: null | {
    title: string,
    description: string,
    settings: CommonValues,
  },
};
