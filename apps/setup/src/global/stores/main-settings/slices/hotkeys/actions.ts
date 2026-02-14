import { createHotkey } from "./actions/create-hotkey.act";
import { editHotkey } from "./actions/edit-hotkey.act";
import { updateHotkey } from "./actions/update-hotkey.act";
import { commitHotkey } from "./actions/commit-hotkey.act";
import { rollbackHotkey } from "./actions/rollback-hotkey.act";
import { deleteHotkey } from "./actions/delete-hotkey.act";

export const HotkeysActions = (set: any) => ({
  createHotkey: createHotkey(set),
  editHotkey: editHotkey(set),
  updateHotkey: updateHotkey(set),
  commitHotkey: commitHotkey(set),
  rollbackHotkey: rollbackHotkey(set),
  deleteHotkey: deleteHotkey(set),
}) as const;

export type HotkeysActions = ReturnType<typeof HotkeysActions>;
