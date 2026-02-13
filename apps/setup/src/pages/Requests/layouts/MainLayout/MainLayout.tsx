import { useUiStore } from "@/global/stores/ui";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/views/ui/resizable";

import { Separator } from "@/views/ui/separator";

interface MainLayoutProps {
  groups: React.ReactNode;
  collection: React.ReactNode;
  workingArea: React.ReactNode;
}

export const MainLayout: React.FC<MainLayoutProps> = (props) => {
  const size = useUiStore((state) => state.collectionSize);
  const setCollectionSize = useUiStore((state) => state.setCollectionSize);

  const layout = [size, 100 - size];

  return (
    <ResizablePanelGroup
      direction="horizontal"
      className="items-stretch"
      onLayout={(sizes: number[]) => setCollectionSize(sizes[0])}
    >
      {props.groups}
      <Separator orientation="vertical" className="w-[1px] h-full" />
      <ResizablePanel className="h-full" defaultSize={layout[0]} minSize={18} maxSize={30}>
        {props.collection}
      </ResizablePanel>
      <ResizableHandle withHandle />
      <ResizablePanel defaultSize={layout[1]}>
        {props.workingArea}
      </ResizablePanel>
    </ResizablePanelGroup>
  );
};
