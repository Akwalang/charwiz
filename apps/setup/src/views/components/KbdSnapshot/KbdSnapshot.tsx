import { KeyboardSnapshot } from "@/global/types/settings/types";

interface KbdSnapshotProps {
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

  return <div className="flex flex-row">{views}</div>;
};

interface KeyProps {
  name: string,
}

const Key: React.FC<KeyProps> = ({ name }) => {
  return <div className="px-3 py-2">{name}</div>
};
