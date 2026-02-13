import { useWorkspaceStore } from "@/global/stores/workspace";
import { getActiveEnvironment } from "@/global/stores/workspace/selectors";

import { IconType } from "@/global/data/icons";

import { Icon } from '@/views/components';
import { Button } from '@/views/ui/button';

import { cn } from "@/utils/react";
import { Separator } from "@/views/ui/separator";

interface GroupButtonProps {
  name: string;
  icon: IconType;
  isActive: boolean;
  onClick?: () => void;
}

export const GroupButton: React.FC<GroupButtonProps> = (props) => {
  const { icon, name, isActive, onClick } = props;

  return (
    <div className="relative w-full pt-[100%] h-0">
      <Button
        variant={isActive ? "default" : "ghost"}
        className={cn(
          "absolute top-0 bottom-0 left-0 right-0 h-auto",
          "flex items-center justify-center",
          isActive && "pointer-events-none",
          !isActive && "hover:bg-secondary hover:text-secondary-foreground",
        )}
        onClick={onClick}
      >
        <Icon icon={icon} size={40} />
      </Button>
    </div>
  );
};

interface GroupsProps {}

export const Groups: React.FC<GroupsProps> = () => {
  const environment = useWorkspaceStore(getActiveEnvironment);
  const groups = useWorkspaceStore((state) => state.groups);

  const changeGroup = useWorkspaceStore((state) => state.changeGroup);

  const items = environment.groupIds.map((id) => {
    const props = {
      name: groups[id].name,
      icon: groups[id].icon,
      isActive: id === environment.activeGroupId,
      onClick: () => changeGroup(id),
    };

    return <GroupButton key={id} {...props} />;
  });

  return (
    <div className="flex flex-col w-[56px] p-[8px] gap-[8px]">
      {items}
      <Separator className="relative w-[80%] left-[10%]" orientation="horizontal" />
      <GroupButton name="Add" icon={IconType.Cross} isActive={false} />
    </div>
  );
};
