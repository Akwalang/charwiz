import { createSwitch } from "./actions/create-switch.act";
import { editSwitch } from "./actions/edit-switch.act";
import { updateSwitch } from "./actions/update-switch.act";
import { commitSwitch } from "./actions/commit-switch.act";
import { rollbackSwitch } from "./actions/rollback-switch.act";
import { deleteSwitch } from "./actions/delete-switch.act";

export const SwitchesActions = (set: any) => ({
  createSwitch: createSwitch(set),
  editSwitch: editSwitch(set),
  updateSwitch: updateSwitch(set),
  commitSwitch: commitSwitch(set),
  rollbackSwitch: rollbackSwitch(set),
  deleteSwitch: deleteSwitch(set),
}) as const;

export type SwitchesActions = ReturnType<typeof SwitchesActions>;
