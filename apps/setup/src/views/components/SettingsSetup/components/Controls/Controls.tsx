import { Button, buttonVariants } from "@/views/ui/button";
import { CountdownButton } from "@/views/ui/countdown-button";

interface ControlsProps {
  id: string,
  isEditing: boolean,
  onEdit: (id: string) => void,
  onSave: (id: string) => void,
  onCancel: (id: string) => void,
  onDelete: (id: string) => void,
}

const createCdProps = (
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

export const Controls: React.FC<ControlsProps> = ({ id, isEditing, onEdit, onSave, onCancel, onDelete }) => {
  const waitingText = (remain: number) => <>Waiting...<span className="inline-block w-[12px] text-center">{remain}</span></>;

  if (isEditing) {
    return (
      <>
        <CountdownButton {...createCdProps("secondary", waitingText, () => onCancel(id))}>Cancel</CountdownButton>
        <Button tabIndex={0} onClick={() => onSave(id)}>Save</Button>
      </>
    );
  } else {
    return (
      <>
        <CountdownButton {...createCdProps("secondary", waitingText, () => onDelete(id))}>Delete</CountdownButton>
        <Button tabIndex={0} onClick={() => onEdit(id)}>Edit</Button>
      </>
    );
  }

};
