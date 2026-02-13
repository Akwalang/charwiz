import { Icon, IconType } from "@/views/components";
import { Button } from "@/views/ui/button";

import { cn } from "@/utils/react";

interface ButtonAddTabProps {
  className?: string;
  onClick: () => void;
}

export const ButtonAddTab: React.FC<ButtonAddTabProps> = (props) => {
  return (
    <div className={cn("flex items-center justify-center h-full", props.className)}>
      <Button
        size="sm"
        variant="ghost"
        className={cn(
          "w-full h-full rounded-none",
          "hover:bg-primary/20 hover:[&>svg]:stroke-primary",
          props.className,
        )}
        onClick={props.onClick}
      >
        <Icon size={24} icon={IconType.Cross} />
      </Button>
    </div>
  );
};
