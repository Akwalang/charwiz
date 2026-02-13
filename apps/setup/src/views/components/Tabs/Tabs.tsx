import * as Base from '@/views/ui/tabs';
import { ScrollArea } from "@/views/ui/scroll-area"

import { cn } from '@/utils/react';

type Tab = {
  name: React.ReactNode;
  value: string;
  content: () => React.ReactNode;
};

interface TabsProps {
  className?: string;
  scrollable?: boolean;
  defaultValue?: string;
  tabs: Tab[];
  addons?: React.ReactNode;
};

const Controls: React.FC<Pick<TabsProps, 'tabs'>> = (props) => {
  const tabs = [];

  for (const [i, tab] of props.tabs.entries()) {
    const className = "inline-flex items-center justify-center whitespace-nowrap rounded-full px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-secondary data-[state=active]:text-secondary-foreground hover:text-muted-foreground/70"

    tabs.push(
      <Base.TabsTrigger key={'t' + i} value={tab.value} className={className}>
        {tab.name}
      </Base.TabsTrigger>
    );
  }

  return tabs;
};

const Contents: React.FC<Pick<TabsProps, 'tabs'>> = (props) => {
  const contents = [];

  for (const [i, tab] of props.tabs.entries()) {
    contents.push(
      <Base.TabsContent key={'c' + i} value={tab.value} className="border-none p-0 outline-none">
        {tab.content()}
      </Base.TabsContent>
    );
  }

  return contents;
};

const Addons: React.FC<Pick<TabsProps, 'addons'>> = (props) => {
  if (!props.addons) return null;

  return <div className="ml-auto mr-4">{props.addons}</div>;
};

export const Tabs: React.FC<TabsProps> = (props) => {
  const controls = <Controls tabs={props.tabs} />;
  const contents = <Contents tabs={props.tabs} />;

  const addons = <Addons addons={props.addons} />;

  const Wrapper = props.scrollable ? ScrollArea : (props: any) => <div {...props} />;

  return (
    <Base.Tabs defaultValue={props.defaultValue} className="h-full flex flex-col">
      <div className={cn("space-between flex items-center m-0.5", props.className)}>
        <Base.TabsList className="h-auto mx-[-4px] p-0 bg-transparent select-none">{controls}</Base.TabsList>
        {addons}
      </div>
      <Wrapper className="grow">
        {contents}
      </Wrapper>
    </Base.Tabs>
  );
};
