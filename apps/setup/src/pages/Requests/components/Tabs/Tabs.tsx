import { MouseEvent } from 'react';

import { useWorkspaceStore } from '@/global/stores/workspace';
import { getActiveGroup } from '@/global/stores/workspace/selectors';

import { HorizontalScrollArea, ScrollBar } from "@/views/ui/scroll-area";

import { Tab } from './components/Tab/Tab';
import { Separator } from './components/Separator/Separator';
import { ButtonAddTab } from './components/ButtonAddTab/ButtonAddTab';
import { ButtonAllTabs } from './components/ButtonAllTabs/ButtonAllTabs';

interface TabsProps {}

export const Tabs: React.FC<TabsProps> = () => {
  const group = useWorkspaceStore(getActiveGroup);
  const pages = useWorkspaceStore((state) => state.pages);

  const closePage = useWorkspaceStore((state) => state.closePage);
  const setActivePage = useWorkspaceStore((state) => state.setActivePage);
  const createNewPage = useWorkspaceStore((state) => state.createNewPage);

  const tabs = group.pageIds.map((id) => {
    const props = {
      id,
      name: pages[id].tab.name,
      badge: pages[id].tab.badge,
      isActive: id === group.activePageId,
      setActivePage,
      closePage,
    };

    return <Tab key={id} {...props} />;
  });

  return (
    <div className="relative flex flex-row h-[44px] w-full">
      <HorizontalScrollArea className="max-w-full grow h-full flex justify-center overflow-visible">
        <div className="flex w-full h-[41px] pr-[77px] items-center [&>div]:last:hidden" children={tabs} />
        <ScrollBar
          className="h-[5px] [&>div]:top-[5px] [&>div]:bg-foreground"
          orientation="horizontal"
        />
      </HorizontalScrollArea>
      <div className="absolute top-0 right-0 flex min-w-[38px] h-full z-1 items-center justify-center bg-background">
        <Separator full={true} />
        <ButtonAddTab className="w-[38px]" onClick={createNewPage} />
        <Separator full={true} />
        <ButtonAllTabs
          className="w-[38px]"
          group={group}
          pages={pages}
          setActivePage={setActivePage}
          closePage={closePage}
        />
      </div>
    </div>
  );
};
