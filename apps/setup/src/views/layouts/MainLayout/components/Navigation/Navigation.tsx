import { Link } from "@/views/components/Link/Link";

import { useLang } from "@/global/hooks";

import { useRouterStore } from "@/global/stores/router/router.store";
import { Page } from "@/global/data/pages";

import { cn } from "@/utils/react";

interface NavigationProps {}

const classActive = "px-4 py-2 block rounded-lg bg-primary text-primary-foreground text-sm transition-colors";
const classInactive = "px-4 py-2 block rounded-lg text-sm hover:bg-secondary/60 transition-colors";

export const Navigation: React.FC<NavigationProps> = () => {
  const lang = useLang((state) => state.menu);

  const page = useRouterStore((state) => state.page);
  
  const pages = [
    { name: lang.home, page: Page.Home },
    { name: lang.autoSwitches, page: Page.Switches },
    { name: lang.hotkeys, page: Page.Hotkeys },
    { name: lang.commands, page: Page.Commands },
    { name: lang.symbols, page: Page.Symbols },
    { name: lang.general, page: Page.General },
  ];

  return (
    <nav className="space-y-1">
      {pages.map((item) =>
        <Link key={item.page} className={page === item.page ? cn(classActive, "cursor-default") : classInactive} to={item.page}>
          {item.name}
        </Link>
      )}
    </nav>
  );
};
