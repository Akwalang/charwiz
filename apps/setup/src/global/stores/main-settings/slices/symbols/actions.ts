import { createSymbol } from "./actions/create-symbol.act";
import { editSymbol } from "./actions/edit-symbol.act";
import { updateSymbol } from "./actions/update-symbol.act";
import { commitSymbol } from "./actions/commit-symbol.act";
import { rollbackSymbol } from "./actions/rollback-symbol.act";
import { deleteSymbol } from "./actions/delete-symbol.act";

export const SymbolsActions = (set: any) => ({
  createSymbol: createSymbol(set),
  editSymbol: editSymbol(set),
  updateSymbol: updateSymbol(set),
  commitSymbol: commitSymbol(set),
  rollbackSymbol: rollbackSymbol(set),
  deleteSymbol: deleteSymbol(set),
}) as const;

export type SymbolsActions = ReturnType<typeof SymbolsActions>;
