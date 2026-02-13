import { ScrollArea, ScrollBar } from "@/views/ui/scroll-area";

import { Tree } from "@/views/components";

interface CollectionProps {}

export const Collection: React.FC<CollectionProps> = (props) => {
  return (
    <div className="relative w-full h-full">
      <div className="absolute w-full h-full">
        <ScrollArea className="h-full">
          <Tree />
          <ScrollBar orientation="horizontal" />
        </ScrollArea>
      </div>
    </div>
  );
};
