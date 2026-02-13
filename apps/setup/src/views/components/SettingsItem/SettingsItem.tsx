import { useState } from "react";

import { Button, buttonVariants } from "@/views/ui/button";
import { CountdownButton } from "@/views/ui/countdown-button";

import { Controls } from "./components/Controls/Controls";

interface SettingsItemProps {
  title: string,
  description: string,
}

export const SettingsItem: React.FC<SettingsItemProps> = (props) => {
  const { 0: isEditing, 1: setIsEditing } = useState<boolean>(false);

  return (
    <div className="w-full px-6 flex flex-col items-stretch">
      <div className="w-full py-4 flex items-center justify-between">
        <div className="flex grow flex-col justify-center">
          <p className="font-medium">{props.title}</p>
          {props.description && <p className="text-sm text-foreground/60">{props.description}</p>}
        </div>
        <div className="flex gap-2">
          <Controls
            isEditing={isEditing}
            onEdit={() => setIsEditing(true)}
            onSave={() => setIsEditing(false)}
            onCancel={() => setIsEditing(false)}
          />
        </div>
      </div>
      {isEditing && <>
        <hr />
        <div className="py-4">
          Settings!
        </div>
      </>}
    </div>
  );
};
