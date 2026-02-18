import { KeyboardSnapshot } from "@/global/types/settings/types";
import { cn } from "@/utils/react";

interface KbdSnapshotProps {
  className?: string,
  snapshot: KeyboardSnapshot,
}

export const KbdSnapshot: React.FC<KbdSnapshotProps> = (props) => {
  const { key, modifiers } = props.snapshot;

  const views: React.ReactNode[] = [];

  key && views.push(<Key key={"0-" + key} name={key} />);

  for (const [idx, key] of modifiers.entries()) {
    views.length && views.push(<div key={(idx + 1) + "+"} className="">+</div>);

    views.push(<Key key={(idx + 1) + "-" + key} name={key} />);
  }

  return <div className={cn("flex flex-row items-center gap-1", props.className)}>{views}</div>;
};

interface KeyProps {
  name: string,
}

const split = (str: string) => str.replace(/([a-z])([A-Z0-9])/g, "$1 $2");

const Key: React.FC<KeyProps> = ({ name }) => {
  return <div className="px-1.5 py-0.5 text-foreground border">{split(name)}</div>
};
