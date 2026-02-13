import { MouseEvent } from "react";

import { Group } from "@/global/stores/workspace";
import { PageWithData } from "@/global/stores/workspace/slices/page/state";

import { Icon, IconType } from "@/views/components";
import * as Base from '@/views/ui/dropdown-menu';
import { Button } from "@/views/ui/button";

import { cn } from "@/utils/react";

interface ButtonAllTabsProps {
  className?: string;
  group: Group;
  pages: Record<string, PageWithData>;
  setActivePage: (id: string) => void;
  closePage: (id: string) => void;
}

export const ButtonAllTabs: React.FC<ButtonAllTabsProps> = (props) => {
  const options = props.group.pageIds.map((id) => {
    const sets = {
      id,
      name: props.pages[id].tab.name,
      setActivePage: props.setActivePage,
      closePage: props.closePage,
      isActive: id === props.group.activePageId,
    };

    return <Option key={id} {...sets} />;
  });

  return (
    <div className={cn("flex items-center justify-center h-full", props.className)}>
      <Base.DropdownMenu>
        <Base.DropdownMenuTrigger asChild>
          <Button
            size="sm"
            variant="ghost"
            className={cn(
              "w-full h-full rounded-none",
              "hover:bg-primary/20 hover:[&>svg]:stroke-primary",
              props.className,
            )}
          >
            <Icon size={24} icon={IconType.ChevronDown} />
          </Button>
        </Base.DropdownMenuTrigger>
        <Base.DropdownMenuContent align="end" className="w-[260px]">
          {options}
        </Base.DropdownMenuContent>
      </Base.DropdownMenu>
    </div>
  );
};

interface OptionProps {
  id: string;
  name: string;
  isActive: boolean;
  setActivePage: (id: string) => void;
  closePage: (id: string) => void;
}

const Option: React.FC<OptionProps> = (props) => {
  const onClick = (event: MouseEvent) => {
    event.preventDefault();
    event.stopPropagation();
  };

  const onMouseDown = (event: MouseEvent<HTMLDivElement>): void => {
    event.preventDefault();
    event.stopPropagation();

    event.button === 0 && props.setActivePage(props.id);
    event.button === 1 && props.closePage(props.id);
  };

  return (
    <Base.DropdownMenuItem onClick={onClick} onMouseDown={onMouseDown}>
      <div className="w-full flex items-center justify-between">
        <div className="text-ellipsis overflow-hidden whitespace-nowrap">{props.name}</div>
        <div className="w-[32px] flex items-center justify-center">{props.isActive && <Icon icon={IconType.Check} size={16} />}</div>
      </div>
    </Base.DropdownMenuItem>
  );
};
