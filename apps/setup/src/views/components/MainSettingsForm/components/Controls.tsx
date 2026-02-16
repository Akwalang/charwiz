import { Button, buttonVariants } from "@/views/ui/button";
import { CountdownButton, CountdownButtonProps } from "@/views/ui/countdown-button";

import { useLang } from "@/global/hooks/useLang";

import { CommonSettings } from "@/global/stores/main-settings/types";

import { isEqual } from "@/utils/is";

interface ControlsProps {
  entity: CommonSettings,
  defaultValue: Pick<CommonSettings, "current" | "editing">,
  editEntity: (id: string) => void,
  saveEntity: (id: string) => void,
  cancelEntity: (id: string) => void,
  deleteEntity: (id: string) => void,
}

const defaultProps: Omit<CountdownButtonProps, "waitingText"> = {
  className: buttonVariants({ variant: "secondary" }),
  tabIndex: 0,
  countdown: 3,
  tickSize: 800,
  onTimeUp: () => {},
};

export const Controls: React.FC<ControlsProps> = ({
  entity,
  defaultValue,
  editEntity,
  saveEntity,
  cancelEntity,
  deleteEntity,
}) => {
  const lang = useLang((state) => state.settings.buttons);

  const props: CountdownButtonProps = { ...defaultProps, waitingText: lang.waiting };

  if (entity.isEditing) {
    props.onTimeUp = () => cancelEntity(entity.id);
    isEqual(entity.current, entity.editing) && (props.countdown = 0);

    return (
      <>
        <CountdownButton {...props}>{lang.cancel}</CountdownButton>
        <Button onClick={() => saveEntity(entity.id)}>{lang.save}</Button>
      </>
    );
  } else {
    props.onTimeUp = () => deleteEntity(entity.id);
    isEqual(entity.current, defaultValue.current) && (props.countdown = 0);

    return (
      <>
        <CountdownButton {...props}>{lang.delete}</CountdownButton>
        <Button onClick={() => editEntity(entity.id)}>{lang.edit}</Button>
      </>
    );
  }
};
