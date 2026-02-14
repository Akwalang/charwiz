import { useRouterStore } from "@/global/stores/router/router.store";
import { Page } from "@/global/data/pages";

import { cn } from "@/utils/react";

interface LinkProps {
  to?: Page;
  className?: string;
  children?: React.ReactNode;
}

export const Link: React.FC<LinkProps> = ({ to, className, children }) => {
  const navigate = useRouterStore((state) => state.navigate);

  return (
    <span className={cn('select-none', to != null && "cursor-pointer"   , className)} tabIndex={0} onClick={() => to && navigate(to)}>
      {children}
    </span>
  );
};
