import { useWorkspaceStore } from '@/global/stores/workspace';

import { Art, ArtName } from '@/views/components';
import { Button } from '@/views/ui/button';

import { ScrollArea } from "@/views/ui/scroll-area";
import { Icon, IconType } from '@/views/components';

interface EmptyGroupProps {}

export const EmptyGroup: React.FC<EmptyGroupProps> = () => {
  const createNewPage = useWorkspaceStore((state) => state.createNewPage);

  return (
    <div className="flex items-center justify-center w-full h-full text-sm text-primary/50">
      <div className="flex flex-col items-center justify-center gap-2">
        <div className="flex">
          <Art className="w-[360px]" ratio={1.4} name={ArtName.Landing} />
          {/* <Art width="240px" ratio={0.96} name={ArtName.BroomBattle} /> */}
          {/* <Art width="360px" ratio={1.18} name={ArtName.BroomBattle2} /> */}
          {/* <Art width="240px" ratio={0.72} name={ArtName.AlienMonocycle} /> */}
          {/* <Art width="360px" ratio={1.55} name={ArtName.Flamethrower} /> */}
          {/* <Art width="170px" ratio={0.81} name={ArtName.Flamethrower2} /> */}
          {/* <Art width="220px" ratio={0.71} name={ArtName.Recreation} /> */}
          {/* <Art width="280px" ratio={1.5} name={ArtName.Motorcycle} /> */}
          {/* <Art width="340px" ratio={1.36} name={ArtName.Doubts} /> */}
          {/* <Art width="300px" ratio={1.11} name={ArtName.Queen} /> */}
          {/* <Art width="160px" ratio={1.11} name={ArtName.RunAway} /> */}
          {/* <Art width="160px" ratio={0.78} name={ArtName.ChickenOut} /> */}
          {/* <Art width="220px" ratio={1.04} name={ArtName.Attack} /> */}
          {/* <Art width="220px" ratio={1.07} name={ArtName.Flying} /> */}
          {/* <Art width="220px" ratio={1.32} name={ArtName.Swimming} /> */}
          {/* <Art width="200px" ratio={1.37} name={ArtName.Defeated} /> */}
          {/* <Art width="240px" ratio={0.78} name={ArtName.Chase} /> */}
        </div>
        <div className="flex select-none">
          <Button variant="secondary" size="lg" onClick={createNewPage}>
            <Icon size={24} icon={IconType.Cross} />New page
          </Button>
        </div>
      </div>
    </div>
  );
};
