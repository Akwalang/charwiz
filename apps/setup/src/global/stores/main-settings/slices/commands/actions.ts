import { createCommand } from "./actions/create-command.act";
import { editCommand } from "./actions/edit-command.act";
import { updateCommand } from "./actions/update-command.act";
import { commitCommand } from "./actions/commit-command.act";
import { rollbackCommand } from "./actions/rollback-command.act";
import { deleteCommand } from "./actions/delete-command.act";

export const CommandsActions = (set: any) => ({
  createCommand: createCommand(set),
  editCommand: editCommand(set),
  updateCommand: updateCommand(set),
  commitCommand: commitCommand(set),
  rollbackCommand: rollbackCommand(set),
  deleteCommand: deleteCommand(set),
}) as const;

export type CommandsActions = ReturnType<typeof CommandsActions>;
