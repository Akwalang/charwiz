import { Symbol } from "@/global/types/settings/settings/main-settings";

export type SymbolItem = {
  id: string,
  isEditing: boolean,
  current: {
    title: string,
    description: string,
    settings: Symbol,
  },
  editing: null | {
    title: string,
    description: string,
    settings: Symbol,
  },
};

export type SymbolsState = {
  symbols: SymbolItem[],
};

export const SymbolsState = (): SymbolsState => ({
  symbols: [],
} as const);
