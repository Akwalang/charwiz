import { Button, buttonVariants } from "@/views/ui/button";
import { CountdownButton } from "@/views/ui/countdown-button";

import { useLang } from "@/global/hooks/useLang";
import { useMainSettingsStore } from "@/global/stores/main-settings";

interface ControlsProps {
  id: string,
  isEditing: boolean,
}

const cdProps = (
  variant: "secondary" | "destructive",
  waitingText: (remain: number) => React.ReactNode,
  onTimeUp: () => void,
) => ({
  className: buttonVariants({ variant }),
  tabIndex: 0,
  countdown: 3,
  tickSize: 800,
  waitingText,
  onTimeUp,
} as const);

export const Controls: React.FC<ControlsProps> = ({ id, isEditing }) => {
  const lang = useLang((state) => state.settings.buttons);

  const editHotkey = useMainSettingsStore((state) => state.editHotkey);
  const saveHotkey = useMainSettingsStore((state) => state.commitHotkey);
  const cancelHotkey = useMainSettingsStore((state) => state.rollbackHotkey);
  const deleteHotkey = useMainSettingsStore((state) => state.deleteHotkey);

  if (isEditing) {
    return (
      <>
        <CountdownButton {...cdProps("secondary", lang.waiting, () => cancelHotkey(id))}>{lang.cancel}</CountdownButton>
        <Button onClick={() => saveHotkey(id)}>{lang.save}</Button>
      </>
    );
  } else {
    return (
      <>
        <CountdownButton {...cdProps("secondary", lang.waiting, () => deleteHotkey(id))}>{lang.delete}</CountdownButton>
        <Button onClick={() => editHotkey(id)}>{lang.edit}</Button>
      </>
    );
  }

};
