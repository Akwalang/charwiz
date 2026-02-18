import { Controls } from "@/views/components";

import { CommandFields } from "../CommandFields/CommandFields";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getCommand } from "@/global/stores/main-settings/selectors";

import { DEFAULT_COMMAND } from "@/global/stores/main-settings/slices/commands/constants";

interface CommandProps {
  id: string,
}

export const Command: React.FC<CommandProps> = ({ id }) => {
  const entity = useMainSettingsStore(getCommand(id))!;

  const controlsActions = {
    editEntity: useMainSettingsStore((state) => state.editCommand),
    saveEntity: useMainSettingsStore((state) => state.commitCommand),
    cancelEntity: useMainSettingsStore((state) => state.rollbackCommand),
    deleteEntity: useMainSettingsStore((state) => state.deleteCommand),
  };

  return (
    <div className="w-full px-6 border rounded-2xl flex flex-col items-stretch">
      <div className="w-full py-4 flex items-center justify-between">
        <div className="flex grow flex-col justify-center">
          <div className="font-medium">From: {entity.current.settings.cmd || "Not set"}</div>
          <div className="font-medium">To: {entity.current.settings.cmd || "Not set"}</div>
        </div>
        <div className="flex gap-2">
          <Controls entity={entity} defaultValue={DEFAULT_COMMAND} {...controlsActions} />
        </div>
      </div>
      {entity.isEditing && <CommandFields id={id} />}
    </div>
  );
};
