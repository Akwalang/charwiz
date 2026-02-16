import { Controls } from "./components/Controls/Controls";
// import { Options } from "./components/Options/Options";

import { CommonValues } from "@/global/types/settings/settings/main-settings";

interface SettingsSetupProps {
  id: string,
  title: string,
  description: string,
  isEditing: boolean,
  values: {
    title: string,
    description: string,
    settings: CommonValues,
  },
  onEdit: (id: string) => void,
  onSave: (id: string) => void,
  onCancel: (id: string) => void,
  onDelete: (id: string) => void,
  onUpdate: (id: string, ...args: any[]) => void,
}

export const SettingsSetup: React.FC<SettingsSetupProps> = (props) => {
  return (
    <div className="w-full px-6 flex flex-col items-stretch">
      <div className="w-full py-4 flex items-center justify-between">
        <div className="flex grow flex-col justify-center">
          <p className="font-medium">{props.title}</p>
          {props.description &&
            <p className="text-sm text-foreground/60">{props.description}</p>
          }
        </div>
        <div className="flex gap-2">
          <Controls
            id={props.id}
            isEditing={props.isEditing}
            onEdit={props.onEdit}
            onSave={props.onSave}
            onCancel={props.onCancel}
            onDelete={props.onDelete}
          />
        </div>
      </div>
      {/* {props.isEditing &&
        <Options id={props.id} values={props.values} onUpdate={props.onUpdate} />
      } */}
    </div>
  );
};
