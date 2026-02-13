import { Button, buttonVariants } from "@/views/ui/button";
import { CountdownButton } from "@/views/ui/countdown-button";

interface ControlsProps {
  isEditing: boolean,
  onEdit: () => void,
  onSave: () => void,
  onCancel: () => void,
}

export const Controls: React.FC<ControlsProps> = ({ isEditing, onEdit, onSave, onCancel }) => {
  if (!isEditing) {
    return <Button tabIndex={0} onClick={onEdit}>Edit</Button>;
  }

  const waitingText = (remain: number) => <>Waiting...<span className="inline-block w-[12px] text-center">{remain}</span></>;

  return (
    <>
      <CountdownButton
        className={buttonVariants({ variant: "secondary" })}
        tabIndex={0}
        countdown={3}
        waitingText={waitingText}
        onTimeUp={onCancel}
      >Cancel</CountdownButton>
      <Button tabIndex={0} onClick={onSave}>Save</Button>
    </>
  );
};
