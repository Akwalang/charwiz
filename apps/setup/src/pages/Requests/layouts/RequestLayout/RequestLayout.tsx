import { useUiStore } from "@/global/stores/ui";

import { useWorkspaceStore } from "@/global/stores/workspace";
import { getActivePage } from "@/global/stores/workspace/selectors";

import * as BaseRS from "@/views/ui/resizable";

import { RequestMain } from '../../components/RequestMain/RequestMain';
import { RequestSettings } from '../../components/RequestSettings/RequestSettings';
import { Response } from '../../components/Response/Response';

interface RequestLayoutProps {}

export const RequestLayout: React.FC<RequestLayoutProps> = () => {
  const page = useWorkspaceStore(getActivePage);

  const size = useUiStore((state) => state.requestSize);
  const setRequestSize = useUiStore((state) => state.setRequestSize);

  const layout = [size, 100 - size];

  if (!page) return null;

  return (
    <div key={page.id} className="flex grow flex-col items-stretch h-full">
      <div className="h-[80px]">
        <RequestMain {...page.data} />
      </div>
      <div className="flex grow">
        <BaseRS.ResizablePanelGroup
          direction="vertical"
          className="items-stretch"
          onLayout={(sizes: number[]) => setRequestSize(sizes[0])}
        >
          <BaseRS.ResizablePanel defaultSize={layout[0]} className="flex flex-col">
            <RequestSettings {...page.data} />
          </BaseRS.ResizablePanel>
          <BaseRS.ResizableHandle withHandle />
          <BaseRS.ResizablePanel defaultSize={layout[1]}>
            <Response />
          </BaseRS.ResizablePanel>
        </BaseRS.ResizablePanelGroup>
      </div>
    </div>
  );
};
